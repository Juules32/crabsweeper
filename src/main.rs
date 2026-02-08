use macroquad::prelude::*;
use minesweeprs::{solve, MineCount, Rule};
use crabsweeper::{BitGrid, Cell, CellContent, CellState, MinesweeperGrid};

const SCALE: usize = 2;
const GRID_SIZE: usize = 16;

fn get_spritesheet_index(cell: &Cell) -> usize {
    match cell.state {
        CellState::Covered => 12,
        CellState::Revealed => {
            match cell.content {
                CellContent::Empty => 0,
                CellContent::Number(n) => n as usize,
                CellContent::Mine => 10,
            }
        }
        CellState::Flagged => 11,
    }
}

fn get_spritesheet_source(cell: &Cell) -> Rect {
    Rect {
        x: (get_spritesheet_index(cell) * GRID_SIZE) as f32,
        y: 0.,
        w: GRID_SIZE as f32,
        h: GRID_SIZE as f32,
    }
}

fn get_grid_point(grid: &MinesweeperGrid, x: isize, y: isize) -> Option<(usize, usize)> {
    let screen_width = screen_width() as isize;
    let screen_height = screen_height() as isize;

    let grid_offset_x = (grid.width * GRID_SIZE * SCALE / 2) as isize;
    let grid_offset_y = (grid.height * GRID_SIZE * SCALE / 2) as isize;

    let offset_x = (x - (screen_width / 2) + grid_offset_x) / GRID_SIZE as isize;
    let offset_y = (y - (screen_height / 2) + grid_offset_y) / GRID_SIZE as isize;
    println!("Grid offset {grid_offset_x} {grid_offset_y}");
    println!("Offset {offset_x} {offset_y}");
    println!();

    if offset_x < 0 || offset_x >= grid.width as isize || offset_y < 0 || offset_y >= grid.height as isize {
        None
    } else {
        Some((offset_x as usize, offset_y as usize))
    }
}

fn get_mouse_cell(grid: &MinesweeperGrid) -> &Cell {
    grid.get(5, 3)
}

fn get_mouse_cell_mut(grid: &mut MinesweeperGrid) -> &mut Cell {
    grid.get_mut(5, 3)
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
        Ok(
            [
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
            ].into(),
        )
    );

    let mut bit_grid = BitGrid::new(8, 6);
    bit_grid.set(2, 0, true);
    bit_grid.set(2, 1, true);
    bit_grid.set(2, 2, true);
    bit_grid.set(2, 3, true);
    bit_grid.set(2, 4, true);
    bit_grid.set(2, 5, true);

    info!("{}", bit_grid);

    let mut minesweeper_grid = MinesweeperGrid::from(bit_grid);
    minesweeper_grid.reveal(4, 5);
    minesweeper_grid.reveal(2, 3);
    minesweeper_grid.reveal(3, 3);
    minesweeper_grid.flag(0, 0);
    info!("{}", minesweeper_grid);

    let texture = load_texture("assets/crabsweeper.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest);

    loop {
        clear_background(DARKBLUE);

        if is_mouse_button_pressed(MouseButton::Left) {
            get_mouse_cell_mut(&mut minesweeper_grid).state = CellState::Flagged;
            info!("{}", minesweeper_grid);
            get_grid_point(&minesweeper_grid, mouse_position().0 as isize, mouse_position().1 as isize);
        }

        for (x, y, cell) in minesweeper_grid.iter_xy() {
            let params = DrawTextureParams {
                dest_size: Some(Vec2 { x: (GRID_SIZE * SCALE) as f32, y: (GRID_SIZE * SCALE) as f32 }),
                source: Some(get_spritesheet_source(cell)),
                ..Default::default()
            };
            draw_texture_ex(&texture, (x * GRID_SIZE * SCALE) as f32, (y * GRID_SIZE * SCALE) as f32, WHITE, params);
        }

        next_frame().await
    }
}
