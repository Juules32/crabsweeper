use macroquad::hash;
use macroquad::miniquad::window::screen_size;
use macroquad::prelude::*;
use macroquad::ui::{root_ui, Skin};
use crate::{hash, Cell, CellContent, CellState, Generator, MinesweeperGame, NaiveSolvableGenerator, OptimizedSolvableGenerator, RandomGenerator, SolveStatus, Solver, State};
use crate::Position;

const MIN_UI_WIDTH: f32 = 100.0;

const CELL_SIZE: f32 = 16.0;
const BOARD_PADDING: f32 = 116.0;
const MIN_ZOOM: f32 = 0.1;
const MAX_ZOOM: f32 = 1.5;
const ZOOM_STEP: f32 = 0.08;
const SCALE_THRESHOLD: f32 = 0.3;

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

fn draw_centered_text(text: &str, y: f32, font_size: f32, color: Color) {
    let t = measure_text(text, None, font_size as u16, 1.0);
    let x = (screen_width() - t.width) / 2.0;
    draw_text(text, x, y, font_size, color);
}

pub struct Presentation {
    zoom_level: f32,
    width_slider: f32,
    height_slider: f32,
    generator_option: usize,
    generator_seed: String,
    generator_num_mines: f32,
    show_settings: bool,
    scale: f32,
    spritesheet: Texture2D,
    pressed_cell: Option<Position>,
    game: MinesweeperGame,
}

impl Presentation {
    pub async fn new() -> Self {
        let game = MinesweeperGame::new(9, 9, Box::new(RandomGenerator { seed: hash(&String::new()), num_mines: 10 }));

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
            width_slider: 9.0,
            height_slider: 9.0,
            generator_option: 0,
            generator_seed: String::new(),
            generator_num_mines: 10.0,
            show_settings: true,
            scale: 1.0,
            spritesheet,
            pressed_cell: None,
            game,
        }
    }

    pub async fn run() {
        let mut presentation = Presentation::new().await;

        loop {
            presentation.update_scale();
            presentation.handle_input();
            presentation.render_grid();
            presentation.render_ui();
            next_frame().await
        }
    }

    fn get_scale(&self) -> f32 {
        let screen_width = screen_width() - BOARD_PADDING * 2.0;
        let screen_height = screen_height() - BOARD_PADDING * 2.0;
        let screen_ratio = screen_width / screen_height;

        let grid_width = self.game.grid.width as f32;
        let grid_height = self.game.grid.height as f32;
        let grid_ratio = grid_width / grid_height;

        let unzoomed_scale = if screen_ratio >= grid_ratio {
            screen_height / (grid_height * CELL_SIZE)
        } else {
            screen_width / (grid_width * CELL_SIZE)
        };
        let initial_scale = unzoomed_scale * self.zoom_level * self.zoom_level;
        if initial_scale < SCALE_THRESHOLD {
            SCALE_THRESHOLD
        } else {
            initial_scale
        }
    }

    fn get_screen_to_grid(&self) -> Option<Position> {
        let (screen_width, screen_height) = screen_size();
        let (mouse_x, mouse_y) = mouse_position();

        let offset_x = self.game.grid.width as f32 * CELL_SIZE * self.scale / 2.0;
        let offset_y = self.game.grid.height as f32 * CELL_SIZE * self.scale / 2.0;

        let grid_x = (mouse_x - (screen_width / 2.0) + offset_x) / (CELL_SIZE * self.scale);
        let grid_y = (mouse_y - (screen_height / 2.0) + offset_y) / (CELL_SIZE * self.scale);

        if grid_x < 0.0 || grid_x >= self.game.grid.width as f32 || grid_y < 0.0 || grid_y >= self.game.grid.height as f32 {
            None
        } else {
            Some(Position { x: grid_x as usize, y: grid_y as usize })
        }
    }

    fn get_grid_to_screen(&self, position: Position) -> Option<(f32, f32)> {
        if position.x >= self.game.grid.width || position.y >= self.game.grid.height {
            return None;
        }

        let (screen_width, screen_height) = screen_size();

        let offset_x = self.game.grid.width as f32 * CELL_SIZE * self.scale / 2.0;
        let offset_y = self.game.grid.height as f32 * CELL_SIZE * self.scale / 2.0;

        let screen_x = position.x as f32 * self.scale * CELL_SIZE + screen_width / 2.0 - offset_x;
        let screen_y = position.y as f32 * self.scale * CELL_SIZE + screen_height / 2.0 - offset_y;

        Some((screen_x, screen_y))
    }

    fn render_ui(&mut self) {
        root_ui().window(hash!(screen_width() as usize, screen_height() as usize, 0), Vec2::new(0.0, 0.0), Vec2::new(screen_width(), screen_height()), |ui| {

            if self.show_settings {
                let size_range = 5.0..30.0;

                ui.group(hash!(screen_width() as usize, screen_height() as usize, 1), Vec2::new(MIN_UI_WIDTH.max((screen_width() / 3.0 - 4.0).round()), 70.0), |ui| {
                    ui.slider(hash!(), "Grid Width", size_range.clone(), &mut self.width_slider);
                    self.width_slider = self.width_slider.round();

                    ui.slider(hash!(), "Grid Height", size_range.clone(), &mut self.height_slider);
                    self.height_slider = self.height_slider.round();

                    let variants = vec!["Random", "Naïve Solvable", "Optimized Solvable"];
                    ui.combo_box(hash!(), "Generator Types", &variants, &mut self.generator_option);
                });

                ui.group(hash!(screen_width() as usize, screen_height() as usize, 2), Vec2::new(MIN_UI_WIDTH.max((screen_width() / 3.0 - 4.0).round()), 70.0), |ui| {
                    ui.input_text(hash(&self.generator_seed), "RNG Seed", &mut self.generator_seed);
                    let random_generator_num_mines_range = 1.0..(self.width_slider * self.height_slider - 9.0).min(99.0);
                    ui.slider(hash!(), "Num Mines", random_generator_num_mines_range, &mut self.generator_num_mines);
                    self.generator_num_mines = self.generator_num_mines.round();
                });

                ui.group(hash!(screen_width() as usize, screen_height() as usize, 3), Vec2::new(MIN_UI_WIDTH.max((screen_width() / 3.0 - 4.0).round()), 70.0), |ui| {
                    if ui.button(None, "Generate Grid") {
                        let generator: Box<dyn Generator> = match self.generator_option {
                            0 => Box::new(RandomGenerator { num_mines: self.generator_num_mines as usize, seed: hash(&self.generator_seed) }),
                            1 => Box::new(NaiveSolvableGenerator { num_mines: self.generator_num_mines as usize, seed: hash(&self.generator_seed) }),
                            2 => Box::new(OptimizedSolvableGenerator { num_mines: self.generator_num_mines as usize, seed: hash(&self.generator_seed) }),
                            _ => panic!("Selected non-existent generator option"),
                        };
                        self.game = MinesweeperGame::new(self.width_slider as usize, self.height_slider as usize, generator);
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

    fn render_grid(&self) {

        clear_background(DARKBLUE);
        for position in self.game.grid.iter_positions() {
            let is_pressed_cell = if let Some(pressed_cell) = self.pressed_cell {
                position == pressed_cell
            } else {
                false
            };
            let params = DrawTextureParams {
                dest_size: Some(Vec2 { x: CELL_SIZE * self.scale, y: CELL_SIZE * self.scale }),
                source: Some(get_spritesheet_source(*self.game.grid.get(position), is_pressed_cell)),
                ..Default::default()
            };
            if let Some((drawn_x, drawn_y)) = self.get_grid_to_screen(position) {
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

    fn handle_input(&mut self) {

        self.pressed_cell = None;
        if let Some(position) = self.get_screen_to_grid() {
            if is_mouse_button_pressed(MouseButton::Right) {
                self.game.flag(position);
            }
            if is_mouse_button_released(MouseButton::Left) {
                self.game.reveal(position);
            }
            if is_mouse_button_down(MouseButton::Left) {
                if self.game.state == State::JustCreated || self.game.state == State::Playing {
                    self.pressed_cell = Some(position);
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

    fn update_scale(&mut self) {
        self.scale = self.get_scale();
    }
}
