use macroquad::prelude::*;
use minesweeprs::{solve, MineCount, Rule};
use crabsweeper::{BitGrid, Cell, CellContent, CellState, MinesweeperGrid};

const SCALE: f32 = 3.0;
const GRID_SIZE: f32 = 16.0;

fn get_spritesheet_index(cell: &Cell) -> f32 {
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

fn get_spritesheet_source(cell: &Cell) -> Rect {
    Rect {
        x: get_spritesheet_index(cell) * GRID_SIZE,
        y: 0.,
        w: GRID_SIZE,
        h: GRID_SIZE,
    }
}

fn get_screen_to_grid(grid: &MinesweeperGrid, screen_position: (f32, f32)) -> Option<(usize, usize)> {
    let screen_width = screen_width();
    let screen_height = screen_height();

    let offset_x = grid.width as f32 * GRID_SIZE * SCALE / 2.0;
    let offset_y = grid.height as f32 * GRID_SIZE * SCALE / 2.0;

    let grid_x = (screen_position.0 - (screen_width / 2.0) + offset_x) / (GRID_SIZE * SCALE);
    let grid_y = (screen_position.1 - (screen_height / 2.0) + offset_y) / (GRID_SIZE * SCALE);

    if grid_x < 0.0 || grid_x >= grid.width as f32 || grid_y < 0.0 || grid_y >= grid.height as f32 {
        None
    } else {
        Some((grid_x as usize, grid_y as usize))
    }
}

fn get_grid_to_screen(grid: &MinesweeperGrid, cell_position: (usize, usize)) -> Option<(f32, f32)> {
    if cell_position.0 >= grid.width || cell_position.1 >= grid.height {
        return None;
    }

    let screen_width = screen_width();
    let screen_height = screen_height();

    let offset_x = grid.width as f32 * GRID_SIZE * SCALE / 2.0;
    let offset_y = grid.height as f32 * GRID_SIZE * SCALE / 2.0;

    let screen_x = cell_position.0 as f32 * SCALE * GRID_SIZE + screen_width / 2.0 - offset_x;
    let screen_y = cell_position.1 as f32 * SCALE * GRID_SIZE + screen_height / 2.0 - offset_y;

    Some((screen_x, screen_y))
}

#[macroquad::main("BasicShapes")]
async fn main() {
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

    let mut bit_grid = BitGrid::new(8, 6);
    bit_grid.set(2, 0, true);
    bit_grid.set(2, 1, true);
    bit_grid.set(2, 2, true);
    bit_grid.set(2, 3, true);
    bit_grid.set(2, 4, true);
    bit_grid.set(2, 5, true);

    info!("{}", bit_grid);

    let mut grid = MinesweeperGrid::from(bit_grid);
    grid.reveal(4, 5);
    grid.reveal(2, 3);
    grid.reveal(3, 3);
    grid.flag(0, 0);
    info!("{}", grid);

    let texture = load_texture("assets/crabsweeper.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest);

    loop {
        clear_background(DARKBLUE);

        get_screen_to_grid(&grid, mouse_position());
        if let Some((x, y)) = get_screen_to_grid(&grid, mouse_position()) {
            if is_mouse_button_pressed(MouseButton::Right) {
                grid.flag(x, y);
            } else if is_mouse_button_pressed(MouseButton::Left) {
                grid.reveal(x, y);
            }
        }

        for (x, y, cell) in grid.iter_xy() {
            let params = DrawTextureParams {
                dest_size: Some(Vec2 { x: GRID_SIZE * SCALE, y: GRID_SIZE * SCALE }),
                source: Some(get_spritesheet_source(cell)),
                ..Default::default()
            };
            if let Some((drawn_x, drawn_y)) = get_grid_to_screen(&grid, (x, y)) {
                draw_texture_ex(&texture, drawn_x, drawn_y, WHITE, params);
            }
        }

        draw_text(&format!("{:?}", mouse_position()), 10.0, 30.0, 30.0, BLACK);

        next_frame().await
    }
}
