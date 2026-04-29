use macroquad::prelude::*;
use crabsweeper::{Cell, CellContent, CellState, MinesweeperGame, MinesweeperGrid, RandomGenerator, State, Solver, SolveStatus, Generator, NaiveSolvableGenerator};
use deterministic_hash::DeterministicHasher;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use macroquad::ui::{hash, root_ui, Skin};

// How many pixels a cell is
const CELL_SIZE: f32 = 16.0;
const BOARD_PADDING: f32 = 116.0;
const MIN_ZOOM: f32 = 0.1;
const MAX_ZOOM: f32 = 1.5;
const ZOOM_STEP: f32 = 0.08;
const SCALE_THRESHOLD: f32 = 0.3;
const MIN_UI_WIDTH: f32 = 100.0;

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

fn window_conf() -> Conf {
    Conf {
        window_title: "Crabsweeper".to_string(),
        window_width: 800,
        window_height: 600,
        window_resizable: true,
        ..Default::default()
    }
}

fn hash<T: Hash>(value: &T) -> u64 {
    let hasher = DefaultHasher::new();
    let mut hasher = DeterministicHasher::new(hasher);
    value.hash(&mut hasher);
    hasher.finish()
}


fn draw_centered_text(text: &str, y: f32, font_size: f32, color: Color) {
    let t = measure_text(text, None, font_size as u16, 1.0);
    let x = (screen_width() - t.width) / 2.0;
    draw_text(text, x, y, font_size, color);
}

#[macroquad::main(window_conf)]
async fn main() {
    let spritesheet: Texture2D = load_texture("assets/spritesheet.png").await.unwrap();
    let mut pressed_cell: Option<(usize, usize)>;

    spritesheet.set_filter(FilterMode::Nearest);

    let solver = Solver;
    let mut zoom_level: f32 = 1.0;
    let mut ui_width: f32 = 9.0;
    let mut ui_height: f32 = 9.0;
    let mut ui_generator_option: usize = 0;
    let mut ui_random_generator_seed = String::new();
    let mut ui_random_generator_num_mines: f32 = 10.0;
    let mut show_settings = true;

    let mut game = MinesweeperGame::new(9, 9, Box::new(RandomGenerator { seed: hash(&ui_random_generator_seed), num_mines: 10 }));

    // Make active and inactive colors the same
    let window_style = root_ui().style_builder()
        .color_inactive(Color::new(0.0, 0.0, 0.0, 0.0))
        .color(Color::new(0.0, 0.0, 0.0, 0.0))
        .build();

    let skin = Skin {
        window_style,
        ..root_ui().default_skin()
    };

    // Apply it
    root_ui().push_skin(&skin);

    loop {
        clear_background(DARKBLUE);
        let scale = get_scale(&game.grid, zoom_level);
        pressed_cell = None;

        if let Some((x, y)) = get_screen_to_grid(&game.grid, mouse_position(), scale) {
            if is_mouse_button_pressed(MouseButton::Right) {
                game.flag(x, y);
            }
            if is_mouse_button_released(MouseButton::Left) {
                game.reveal(x, y);
            }
            if is_mouse_button_down(MouseButton::Left) {
                if game.state == State::JustCreated || game.state == State::Playing {
                    pressed_cell = Some((x, y));
                }
            }
        }
        let (_, wheel_y) = mouse_wheel();
        if wheel_y != 0.0 {
            if !(wheel_y.is_sign_negative() && scale <= SCALE_THRESHOLD) {
                zoom_level += wheel_y.signum() * ZOOM_STEP;
                zoom_level = zoom_level.clamp(MIN_ZOOM, MAX_ZOOM);
            }
        }

        for (x, y, cell) in game.grid.iter_xy() {
            let is_pressed_cell = if let Some(pressed_cell) = pressed_cell {
                (x, y) == pressed_cell
            } else {
                false
            };
            let params = DrawTextureParams {
                dest_size: Some(Vec2 { x: CELL_SIZE * scale, y: CELL_SIZE * scale }),
                source: Some(get_spritesheet_source(*cell, is_pressed_cell)),
                ..Default::default()
            };
            if let Some((drawn_x, drawn_y)) = get_grid_to_screen(&game.grid, (x, y), scale) {
                draw_texture_ex(&spritesheet, drawn_x, drawn_y, WHITE, params);
            }
        }

        root_ui().window(hash!(screen_width() as usize, screen_height() as usize, 0), Vec2::new(0.0, 0.0), Vec2::new(screen_width(), screen_height()), |ui| {

            if show_settings {
                let ui_size_range = 5.0..30.0;

                ui.group(hash!(screen_width() as usize, screen_height() as usize, 1), Vec2::new(MIN_UI_WIDTH.max((screen_width() / 3.0 - 4.0).round()), 70.0), |ui| {
                    ui.slider(hash!(), "Grid Width", ui_size_range.clone(), &mut ui_width);
                    ui_width = ui_width.round();

                    ui.slider(hash!(), "Grid Height", ui_size_range.clone(), &mut ui_height);
                    ui_height = ui_height.round();

                    let variants = vec!["Random", "Naïve Solvable"];
                    ui.combo_box(hash!(), "Generator Types", &variants, &mut ui_generator_option);
                });

                ui.group(hash!(screen_width() as usize, screen_height() as usize, 2), Vec2::new(MIN_UI_WIDTH.max((screen_width() / 3.0 - 4.0).round()), 70.0), |ui| {
                    ui.input_text(hash(&ui_random_generator_seed), "RNG Seed", &mut ui_random_generator_seed);
                    let ui_random_generator_num_mines_range = 1.0..(ui_width * ui_height - 9.0).min(99.0);
                    ui.slider(hash!(), "Num Mines", ui_random_generator_num_mines_range, &mut ui_random_generator_num_mines);
                    ui_random_generator_num_mines = ui_random_generator_num_mines.round();
                });

                ui.group(hash!(), Vec2::new(MIN_UI_WIDTH.max((screen_width() / 3.0 - 4.0).round()), 70.0), |ui| {
                    if ui.button(None, "Generate Grid") {
                        let generator: Box<dyn Generator> = match ui_generator_option {
                            0 => Box::new(RandomGenerator { num_mines: ui_random_generator_num_mines as usize, seed: hash(&ui_random_generator_seed) }),
                            1 => Box::new(NaiveSolvableGenerator { num_mines: ui_random_generator_num_mines as usize, seed: hash(&ui_random_generator_seed) }),
                            _ => panic!("Selected non-existent generator option"),
                        };
                        game = MinesweeperGame::new(ui_width as usize, ui_height as usize, generator);
                    }
                    if ui.button(None, "Hide Settings") {
                        show_settings = false;
                    }
                });
            } else {
                if ui.button(None, "Show Settings") {
                    show_settings = true;
                }
            }

            if game.state == State::Playing {
                if ui.button(None, "Solve One Step") {
                    match solver.solve_one_step(&mut game.grid) {
                        Ok(SolveStatus::Stuck) => { println!("Grid contains 50/50s") }
                        Ok(SolveStatus::ProgressMade) => {}
                        Ok(SolveStatus::Won) => { game.state = State::YouWon; }
                        Err(_) => { println!("Something went wrong with the solver"); }
                    }
                }
                if ui.button(None, "Full Solve") {
                    match solver.solve(&mut game.grid) {
                        Ok(SolveStatus::Stuck) => { println!("Grid contains 50/50s") }
                        Ok(SolveStatus::ProgressMade) => {}
                        Ok(SolveStatus::Won) => { game.state = State::YouWon; }
                        Err(_) => { println!("Something went wrong with the solver"); }
                    }
                }
                ui.label(None, &format!("{:02} unflagged crabs left", game.grid.count_remaining_flags()));
            }
            //ui.label(None, &format!("{:?}", mouse_position()));

            if ui.button(Vec2::new(2.0, screen_height() - 22.0), "+") {
                    zoom_level += ZOOM_STEP;
                    zoom_level = zoom_level.clamp(MIN_ZOOM, MAX_ZOOM);
            }
            if ui.button(Vec2::new(15.0, screen_height() - 22.0), "-") {
                if scale > SCALE_THRESHOLD {
                    zoom_level -= ZOOM_STEP;
                    zoom_level = zoom_level.clamp(MIN_ZOOM, MAX_ZOOM);
                }
            }
        });

        if game.state == State::GameOver {
            draw_centered_text("Game Over", screen_height() - 50.0, 50.0, RED);
        }

        if game.state == State::YouWon {
            draw_centered_text("You Won", screen_height() - 50.0, 50.0, GREEN);
        }

        next_frame().await
    }
}
