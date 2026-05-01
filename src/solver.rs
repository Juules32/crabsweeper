use minesweeprs::{InconsistencyError, MineCount, Rule};
use crate::{CellContent, CellState, MinesweeperGrid, Position};

const OTHER_TAG: Position = Position { x: 1000, y: 1000 };
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
        let revealed_number_positions: Vec<Position> = grid
            .iter_positions()
            .filter(|&position| {
                let cell = grid.get(position);
                cell.state == CellState::Revealed && matches!(cell.content, CellContent::Number(_))
            })
            .collect();

        let covered_neighbor_positions: Vec<Vec<Position>> = revealed_number_positions
            .iter()
            .map(|&position| {
                grid.neighbor_positions(position)
                    .iter()
                    .filter_map(|&position| {
                        let cell = grid.get(position);
                        if cell.state == CellState::Covered || cell.state == CellState::Flagged {
                            Some(position)
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .collect();

        let rules: Vec<Rule<Position>> = revealed_number_positions
            .iter()
            .zip(covered_neighbor_positions)
            .filter(|(_, covered_neighbors)| {
                !covered_neighbors.is_empty()
            })
            .map(|(&position, covered_neighbors)| Rule::new(
                if let CellContent::Number(n) = grid.get(position).content {
                    n as usize
                } else {
                    0
                },
                covered_neighbors
            ))
            .collect();

        let total_cells = grid.size();
        let total_mines = grid.count_mines();
        let output = minesweeprs::solve(
            &rules,
            MineCount { total_cells, total_mines },
            &OTHER_TAG
        )?;

        //println!("{rules:?}");
        //println!("{output:?}");

        for (position, value) in output {
            if position != OTHER_TAG {
                if value == 0.0 {
                    grid.reveal(position);
                    is_stuck = false;
                }
                if value == 1.0 {
                    if !grid.get(position).is_flagged() {
                        grid.flag(position);
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
