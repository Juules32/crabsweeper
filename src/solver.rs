use minesweeprs::{MineCount, Rule};
use crate::{Cell, CellContent, CellState, MinesweeperGame, State};

const OTHER_TAG: (usize, usize) = (1000, 1000);

pub struct Solver;

impl Solver {
    pub fn solve(&self, game: &mut MinesweeperGame) {
        let revealed_number_coords: Vec<(usize, usize, &Cell)> = game.grid
            .iter_xy()
            .filter(|&(_, _, cell)| cell.state == CellState::Revealed && matches!(cell.content, CellContent::Number(_)))
            .collect();

        let covered_neighbor_coords: Vec<Vec<(usize, usize)>> = revealed_number_coords
            .iter()
            .map(|&(x, y, _)| {
                game.grid.neighbor_coords(x, y)
                    .iter()
                    .filter_map(|&(x, y)| {
                        let cell = game.grid.get(x, y);
                        if cell.state == CellState::Covered || cell.state == CellState::Flagged {
                            Some((x, y))
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .collect();

        let rules: Vec<Rule<(usize, usize)>> = revealed_number_coords
            .iter()
            .zip(covered_neighbor_coords)
            .filter(|(_, covered_neighbors)| {
                !covered_neighbors.is_empty()
            })
            .map(|(&(_, _, cell), covered_neighbors)| Rule::new(
                if let CellContent::Number(n) = cell.content {
                    n as usize
                } else {
                    0
                },
                covered_neighbors
            ))
            .collect();

        let total_cells = game.grid.cells.len();
        let total_mines = game.grid.count_mines();
        let output = minesweeprs::solve(
            &rules,
            MineCount { total_cells, total_mines },
            &OTHER_TAG
        );

        //println!("{rules:?}");
        //println!("{output:?}");

        if let Ok(output) = output {
            for (key, value) in output {
                let (x, y) = key;
                if key != OTHER_TAG {
                    if value == 0.0 {
                        game.grid.reveal(x, y);
                    }
                    if value == 1.0 {
                        if !game.grid.get(x, y).is_flagged() {
                            game.grid.flag(x, y);
                        }
                    }
                }
            }

            if game.grid.is_solved() {
                game.state = State::YouWon;
            }
        }
    }
}
