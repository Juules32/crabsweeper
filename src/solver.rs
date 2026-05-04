use minesweeprs::InconsistencyError;
use crate::{MinesweeperGrid, Position};

const OTHER_TAG: Position = Position { x: 1000, y: 1000 };
const MAX_SOLVING_ITERATIONS: usize = 100;

#[derive(PartialEq)]
pub enum SolveStatus {
    Stuck,
    ProgressMade,
    Won,
}

pub struct Solver;

impl Solver {
    #[cfg(feature = "custom_rule_generator")]
    pub fn solve_one_step(grid: &mut MinesweeperGrid) -> Result<SolveStatus, InconsistencyError> {
        use crate::{CellState, CellContent};
        use minesweeprs::{Rule, MineCount};
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

    #[cfg(not(feature = "custom_rule_generator"))]
    pub fn solve_one_step(grid: &mut MinesweeperGrid) -> Result<SolveStatus, InconsistencyError> {
        use minesweeprs::util::Board;

        let mut is_stuck = true;

        let minesweeprs_board = Board::new(&grid.to_minesweeprs_string())
            .map_err(|_| InconsistencyError("Something went wrong when creating a minesweeprs board from a string slice"))?;

        let (rules, mine_count) = minesweeprs_board.generate_rules(grid.count_mines(), false);

        let output = minesweeprs::solve(
            &rules,
            mine_count,
            &(OTHER_TAG.x, OTHER_TAG.y),
        )?;

        for ((x, y), value) in output {
            let position = Position { x, y };
            if position != OTHER_TAG {
                if value == 0.0 {
                    grid.reveal(position);
                    is_stuck = false;
                }
                if value == 1.0 && !grid.get(position).is_flagged() {
                    grid.flag(position);
                    is_stuck = false;
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
        Self::solve(&mut grid.clone()) == Ok(SolveStatus::Won)
    }
}
