use macroquad::prelude::*;
use minesweeprs::{solve, MineCount, Rule};
use crabsweeper::{Bitmap, Grid};

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

    println!("{:?}", output);
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

    let mut bitmap = Bitmap::new(8, 6);
    bitmap.set(2, 3, true);
    println!("{bitmap}");

    let mut grid = Grid::from(bitmap);
    grid.reveal(4, 5);
    grid.reveal(2, 3);
    grid.reveal(3, 3);
    grid.flag(0, 0);
    println!("{grid}");

    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
