use minesweeprs::{InconsistencyError, MineCount, Rule};
use crate::{Cell, CellContent, CellState, MinesweeperGrid};

const OTHER_TAG: (usize, usize) = (1000, 1000);
const MAX_SOLVING_ITERATIONS: usize = 100;

pub enum SolveStatus {
    Stuck,
    ProgressMade,
    Won,
}

pub struct Solver;

impl Solver {
    pub fn solve_one_step(grid: &mut MinesweeperGrid) -> Result<SolveStatus, InconsistencyError> {
        let mut is_stuck = true;
        let revealed_number_coords: Vec<(usize, usize, &Cell)> = grid
            .iter_xy()
            .filter(|&(_, _, cell)| cell.state == CellState::Revealed && matches!(cell.content, CellContent::Number(_)))
            .collect();

        let covered_neighbor_coords: Vec<Vec<(usize, usize)>> = revealed_number_coords
            .iter()
            .map(|&(x, y, _)| {
                grid.neighbor_coords(x, y)
                    .iter()
                    .filter_map(|&(x, y)| {
                        let cell = grid.get(x, y);
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

        let total_cells = grid.cells.len();
        let total_mines = grid.count_mines();
        let output = minesweeprs::solve(
            &rules,
            MineCount { total_cells, total_mines },
            &OTHER_TAG
        )?;

        //println!("{rules:?}");
        //println!("{output:?}");

        for (key, value) in output {
            let (x, y) = key;
            if key != OTHER_TAG {
                if value == 0.0 {
                    grid.reveal(x, y);
                    is_stuck = false;
                }
                if value == 1.0 {
                    if !grid.get(x, y).is_flagged() {
                        grid.flag(x, y);
                        is_stuck = false;
                    }
                }
            }
        }

        if grid.is_solved() {
            Ok(SolveStatus::Won)
        } else {
            if is_stuck {
                Ok(SolveStatus::Stuck)
            } else {
                Ok(SolveStatus::ProgressMade)
            }
        }
    }

    pub fn solve(grid: &mut MinesweeperGrid) -> Result<SolveStatus, InconsistencyError> {
        for _ in 0..MAX_SOLVING_ITERATIONS {
            match Self::solve_one_step(grid) {
                Ok(SolveStatus::Stuck) => { return Ok(SolveStatus::Stuck) },
                Ok(SolveStatus::ProgressMade) => {},
                Ok(SolveStatus::Won) => { return Ok(SolveStatus::Won); }
                Err(err) => { return Err(err); },
            }
        }
        Err(InconsistencyError("Solver took too many iterations"))
    }

    pub fn is_solvable(grid: &MinesweeperGrid) -> bool {
        let mut grid = grid.clone();
        match Self::solve(&mut grid) {
            Ok(SolveStatus::Won) => true,
            _ => false,
        }
    }
}
