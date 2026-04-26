use std::cmp::max;
use macroquad::prelude::*;
use minesweeprs::{solve, MineCount, Rule};
use crabsweeper::{Cell, CellContent, CellState, Game, MinesweeperGrid, RandomGenerator};

use macroquad::ui::widgets::{ComboBox, Slider};
use macroquad::ui::{hash, root_ui};

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

fn window_conf() -> Conf {
    Conf {
        window_title: "Crabsweeper".to_string(),
        window_width: 800,
        window_height: 600,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let spritesheet: Texture2D = load_texture("assets/spritesheet.png").await.unwrap();
    let mut pressed_cell: Option<(usize, usize)>;

    spritesheet.set_filter(FilterMode::Nearest);

    let output = solve(
        &[
            Rule::new(1, [0, 1]),
            Rule::new(2, [0, 1, 2]),
            Rule::new(3, [1, 2, 3]),
            Rule::new(2, [2, 3, 4]),
            Rule::new(2, [3, 4, 5, 6, 7]),
            Rule::new(1, [6, 7, 8]),
            Rule::new(1, [7, 8]),
        ],
        MineCount { total_cells: 85, total_mines: 10 },
        &-1,
    );
    info!("{:?}", output);

    // The board is solvable, so the below should hold:
    assert_eq!(
        output,
        Ok([
            (0, 0.0),
            (1, 1.0),
            (2, 1.0),
            (3, 1.0),
            (4, 0.0),
            (5, 0.07792207792207793),
            (6, 0.0),
            (7, 0.9220779220779222),
            (8, 0.07792207792207793),
            (-1, 0.07792207792207792),
        ].into()),
    );

    let mut game = Game::new(6, 3, RandomGenerator {seed: 2, num_mines:3});
    let mut zoom_level: f32 = 1.0;
    let mut ui_width: f32 = 30.0;
    let mut ui_height: f32 = 16.0;
    let mut ui_generator_option: usize = 0;
    let mut ui_random_generator_seed = String::new();
    let mut ui_random_generator_num_mines: usize = 99;

    loop {
        clear_background(DARKBLUE);
        let scale = get_scale(&game.grid, zoom_level);
        pressed_cell = None;

        if let Some((x, y)) = get_screen_to_grid(&game.grid, mouse_position(), scale) {
            if is_mouse_button_pressed(MouseButton::Right) {
                game.flag(x, y);
            }
            if is_mouse_button_released(MouseButton::Left) {
                game.press_cell(x, y);
            }
            if is_mouse_button_down(MouseButton::Left) {
                pressed_cell = Some((x, y));
            }
        }
        if let (_, wheel_y) = mouse_wheel() {
            if wheel_y != 0.0 {
                if !(wheel_y.is_sign_negative() && scale <= SCALE_THRESHOLD) {
                    zoom_level += wheel_y.signum() * ZOOM_STEP;
                    zoom_level = zoom_level.clamp(MIN_ZOOM, MAX_ZOOM);
                }
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

        draw_text(&format!("{:?}", mouse_position()), 10.0, 30.0, 30.0, BLACK);


        root_ui().window(hash!(), Vec2::new(0.0, 0.0), Vec2::new(screen_width(), 100.0), |ui| {
            let ui_size_range = 10f32..100f32;
            Slider::new(hash!(), ui_size_range.clone())
                .label("Grid Width")
                .ui(ui, &mut ui_width);
            ui_width = ui_width.round();

            Slider::new(hash!(), ui_size_range.clone())
                .label("Grid Height")
                .ui(ui, &mut ui_height);
            ui_height = ui_height.round();

            let variants = vec!["RandomGenerator", "Scoop"];
            ComboBox::new(hash!(), &variants)
                .label("Generator Type")
                .ui(ui, &mut ui_generator_option);

            if ui.button(None, "Generate Grid") {
                let random_generator = RandomGenerator {
                    seed: hash!(ui_random_generator_seed.clone()),
                    num_mines: ui_random_generator_num_mines,
                };
                game = Game::new(ui_width as usize, ui_height as usize, random_generator);
            }
        });

        next_frame().await
    }
}
