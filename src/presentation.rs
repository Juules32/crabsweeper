use macroquad::hash;
use macroquad::prelude::*;
use macroquad::ui::{root_ui, Skin};
use crate::{hash, Cell, CellContent, CellState, Generator, MinesweeperGame, MinesweeperGrid, NaiveSolvableGenerator, OptimizedSolvableGenerator, RandomGenerator, SolveStatus, Solver, State};

const MIN_UI_WIDTH: f32 = 100.0;

// How many pixels a cell is
const CELL_SIZE: f32 = 16.0;
const BOARD_PADDING: f32 = 116.0;
const MIN_ZOOM: f32 = 0.1;
const MAX_ZOOM: f32 = 1.5;
const ZOOM_STEP: f32 = 0.08;
const SCALE_THRESHOLD: f32 = 0.3;

// Ratio of screen pixel to sprite pixel
fn get_scale(grid: &MinesweeperGrid, zoom_level: f32) -> f32 {
    let screen_width = screen_width() - BOARD_PADDING * 2.0;
    let screen_height = screen_height() - BOARD_PADDING * 2.0;
    let screen_ratio = screen_width / screen_height;

    let grid_width = grid.width as f32;
    let grid_height = grid.height as f32;
    let grid_ratio = grid_width / grid_height;

    let unzoomed_scale = if screen_ratio >= grid_ratio {
        screen_height / (grid_height * CELL_SIZE)
    } else {
        screen_width / (grid_width * CELL_SIZE)
    };
    let initial_scale = unzoomed_scale * zoom_level * zoom_level;
    if initial_scale < SCALE_THRESHOLD {
        SCALE_THRESHOLD
    } else {
        initial_scale
    }
}

fn get_spritesheet_index(cell: Cell, is_pressed_cell: bool) -> f32 {
    if is_pressed_cell && cell.state == CellState::Covered {
        0.0
    } else {
        match cell.state {
            CellState::Covered => 12.0,
            CellState::Revealed => {
                match cell.content {
                    CellContent::Empty => 0.0,
                    CellContent::Number(n) => n as f32,
                    CellContent::Mine => 10.0,
                }
            }
            CellState::Flagged => 11.0,
        }
    }
}

fn get_spritesheet_source(cell: Cell, is_pressed_cell: bool) -> Rect {
    Rect {
        x: get_spritesheet_index(cell, is_pressed_cell) * CELL_SIZE,
        y: 0.0,
        w: CELL_SIZE,
        h: CELL_SIZE,
    }
}

fn get_screen_to_grid(grid: &MinesweeperGrid, screen_position: (f32, f32), scale: f32) -> Option<(usize, usize)> {
    let screen_width = screen_width();
    let screen_height = screen_height();

    let offset_x = grid.width as f32 * CELL_SIZE * scale / 2.0;
    let offset_y = grid.height as f32 * CELL_SIZE * scale / 2.0;

    let grid_x = (screen_position.0 - (screen_width / 2.0) + offset_x) / (CELL_SIZE * scale);
    let grid_y = (screen_position.1 - (screen_height / 2.0) + offset_y) / (CELL_SIZE * scale);

    if grid_x < 0.0 || grid_x >= grid.width as f32 || grid_y < 0.0 || grid_y >= grid.height as f32 {
        None
    } else {
        Some((grid_x as usize, grid_y as usize))
    }
}

fn get_grid_to_screen(grid: &MinesweeperGrid, cell_position: (usize, usize), scale: f32) -> Option<(f32, f32)> {
    if cell_position.0 >= grid.width || cell_position.1 >= grid.height {
        return None;
    }

    let screen_width = screen_width();
    let screen_height = screen_height();

    let offset_x = grid.width as f32 * CELL_SIZE * scale / 2.0;
    let offset_y = grid.height as f32 * CELL_SIZE * scale / 2.0;

    let screen_x = cell_position.0 as f32 * scale * CELL_SIZE + screen_width / 2.0 - offset_x;
    let screen_y = cell_position.1 as f32 * scale * CELL_SIZE + screen_height / 2.0 - offset_y;

    Some((screen_x, screen_y))
}


fn draw_centered_text(text: &str, y: f32, font_size: f32, color: Color) {
    let t = measure_text(text, None, font_size as u16, 1.0);
    let x = (screen_width() - t.width) / 2.0;
    draw_text(text, x, y, font_size, color);
}

pub struct Presentation {
    zoom_level: f32,
    ui_width: f32,
    ui_height: f32,
    ui_generator_option: usize,
    ui_random_generator_seed: String,
    ui_random_generator_num_mines: f32,
    show_settings: bool,
    scale: f32,
    spritesheet: Texture2D,
    pressed_cell: Option<(usize, usize)>,
    game: MinesweeperGame,
}

impl Presentation {
    pub async fn new(game: MinesweeperGame) -> Self {
        // todo: make init instead of new? single responsibility principle?
        let spritesheet = load_texture("assets/spritesheet.png").await.unwrap();
        spritesheet.set_filter(FilterMode::Nearest);

        let window_style = root_ui().style_builder()
            .color_inactive(Color::new(0.0, 0.0, 0.0, 0.0))
            .color(Color::new(0.0, 0.0, 0.0, 0.0))
            .build();
        let skin = Skin {
            window_style,
            ..root_ui().default_skin()
        };
        root_ui().push_skin(&skin);

        Self {
            zoom_level: 1.0,
            ui_width: 9.0,
            ui_height: 9.0,
            ui_generator_option: 0,
            ui_random_generator_seed: String::new(),
            ui_random_generator_num_mines: 10.0,
            show_settings: true,
            scale: get_scale(&game.grid, 1.0),
            spritesheet,
            pressed_cell: None,
            game,
        }
    }

    pub async fn run(&mut self) {
        loop {
            self.update_scale();
            self.handle_input();
            self.render_grid();
            self.render_ui();
            next_frame().await
        }
    }

    pub fn render_ui(&mut self) {
        root_ui().window(hash!(screen_width() as usize, screen_height() as usize, 0), Vec2::new(0.0, 0.0), Vec2::new(screen_width(), screen_height()), |ui| {

            if self.show_settings {
                let ui_size_range = 5.0..30.0;

                ui.group(hash!(screen_width() as usize, screen_height() as usize, 1), Vec2::new(MIN_UI_WIDTH.max((screen_width() / 3.0 - 4.0).round()), 70.0), |ui| {
                    ui.slider(hash!(), "Grid Width", ui_size_range.clone(), &mut self.ui_width);
                    self.ui_width = self.ui_width.round();

                    ui.slider(hash!(), "Grid Height", ui_size_range.clone(), &mut self.ui_height);
                    self.ui_height = self.ui_height.round();

                    let variants = vec!["Random", "Naïve Solvable", "Optimized Solvable"];
                    ui.combo_box(hash!(), "Generator Types", &variants, &mut self.ui_generator_option);
                });

                ui.group(hash!(screen_width() as usize, screen_height() as usize, 2), Vec2::new(MIN_UI_WIDTH.max((screen_width() / 3.0 - 4.0).round()), 70.0), |ui| {
                    ui.input_text(hash(&self.ui_random_generator_seed), "RNG Seed", &mut self.ui_random_generator_seed);
                    let ui_random_generator_num_mines_range = 1.0..(self.ui_width * self.ui_height - 9.0).min(99.0);
                    ui.slider(hash!(), "Num Mines", ui_random_generator_num_mines_range, &mut self.ui_random_generator_num_mines);
                    self.ui_random_generator_num_mines = self.ui_random_generator_num_mines.round();
                });

                ui.group(hash!(screen_width() as usize, screen_height() as usize, 3), Vec2::new(MIN_UI_WIDTH.max((screen_width() / 3.0 - 4.0).round()), 70.0), |ui| {
                    if ui.button(None, "Generate Grid") {
                        let generator: Box<dyn Generator> = match self.ui_generator_option {
                            0 => Box::new(RandomGenerator { num_mines: self.ui_random_generator_num_mines as usize, seed: hash(&self.ui_random_generator_seed) }),
                            1 => Box::new(NaiveSolvableGenerator { num_mines: self.ui_random_generator_num_mines as usize, seed: hash(&self.ui_random_generator_seed) }),
                            2 => Box::new(OptimizedSolvableGenerator { num_mines: self.ui_random_generator_num_mines as usize, seed: hash(&self.ui_random_generator_seed) }),
                            _ => panic!("Selected non-existent generator option"),
                        };
                        self.game = MinesweeperGame::new(self.ui_width as usize, self.ui_height as usize, generator);
                    }
                    if ui.button(None, "Hide Settings") {
                        self.show_settings = false;
                    }
                });
            } else {
                if ui.button(None, "Show Settings") {
                    self.show_settings = true;
                }
            }

            if self.game.state == State::Playing {
                if ui.button(None, "Solve One Step") {
                    match Solver::solve_one_step(&mut self.game.grid) {
                        Ok(SolveStatus::Stuck) => { self.game.status_message = String::from("Grid contains 50/50s"); }
                        Ok(SolveStatus::ProgressMade) => {}
                        Ok(SolveStatus::Won) => { self.game.state = State::YouWon; }
                        Err(_) => { self.game.status_message = String::from("Something went wrong with the solver"); }
                    }
                }
                if ui.button(None, "Full Solve") {
                    match Solver::solve(&mut self.game.grid) {
                        Ok(SolveStatus::Stuck) => { self.game.status_message = String::from("Grid contains 50/50s"); }
                        Ok(SolveStatus::ProgressMade) => {}
                        Ok(SolveStatus::Won) => { self.game.state = State::YouWon; }
                        Err(_) => { self.game.status_message = String::from("Something went wrong with the solver"); }
                    }
                }
                ui.label(None, &format!("{:02} unflagged crabs left", self.game.grid.count_remaining_flags()));
                ui.label(None, &self.game.status_message);
            }
            //ui.label(None, &format!("{:?}", mouse_position()));

            if ui.button(Vec2::new(2.0, screen_height() - 22.0), "+") {
                self.zoom_level += ZOOM_STEP;
                self.zoom_level = self.zoom_level.clamp(MIN_ZOOM, MAX_ZOOM);
            }
            if ui.button(Vec2::new(15.0, screen_height() - 22.0), "-") {
                if self.scale > SCALE_THRESHOLD {
                    self.zoom_level -= ZOOM_STEP;
                    self.zoom_level = self.zoom_level.clamp(MIN_ZOOM, MAX_ZOOM);
                }
            }
        });
    }

    pub fn render_grid(&self) {

        clear_background(DARKBLUE);
        for (x, y, cell) in self.game.grid.iter_xy() {
            let is_pressed_cell = if let Some(pressed_cell) = self.pressed_cell {
                (x, y) == pressed_cell
            } else {
                false
            };
            let params = DrawTextureParams {
                dest_size: Some(Vec2 { x: CELL_SIZE * self.scale, y: CELL_SIZE * self.scale }),
                source: Some(get_spritesheet_source(*cell, is_pressed_cell)),
                ..Default::default()
            };
            if let Some((drawn_x, drawn_y)) = get_grid_to_screen(&self.game.grid, (x, y), self.scale) {
                draw_texture_ex(&self.spritesheet, drawn_x, drawn_y, WHITE, params);
            }
        }

        if self.game.state == State::GameOver {
            draw_centered_text("Game Over", screen_height() - 50.0, 50.0, RED);
        }

        if self.game.state == State::YouWon {
            draw_centered_text("You Won", screen_height() - 50.0, 50.0, GREEN);
        }
    }

    pub fn handle_input(&mut self) {

        self.pressed_cell = None;
        if let Some((x, y)) = get_screen_to_grid(&self.game.grid, mouse_position(), self.scale) {
            if is_mouse_button_pressed(MouseButton::Right) {
                self.game.flag(x, y);
            }
            if is_mouse_button_released(MouseButton::Left) {
                self.game.reveal(x, y);
            }
            if is_mouse_button_down(MouseButton::Left) {
                if self.game.state == State::JustCreated || self.game.state == State::Playing {
                    self.pressed_cell = Some((x, y));
                }
            }
        }
        let (_, wheel_y) = mouse_wheel();
        if wheel_y != 0.0 {
            if !(wheel_y.is_sign_negative() && self.scale <= SCALE_THRESHOLD) {
                self.zoom_level += wheel_y.signum() * ZOOM_STEP;
                self.zoom_level = self.zoom_level.clamp(MIN_ZOOM, MAX_ZOOM);
            }
        }

    }

    pub fn update_scale(&mut self) {
        self.scale = get_scale(&self.game.grid, self.zoom_level);
    }
}
