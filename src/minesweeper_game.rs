use crate::{CellContent, CellState, Generator, MinesweeperGrid, Position};

#[derive(PartialEq)]
pub enum State {
    JustCreated,
    Playing,
    GameOver,
    YouWon,
}

pub struct MinesweeperGame {
    pub state: State,
    pub grid: MinesweeperGrid,
    generator: Box<dyn Generator>,
    pub status_message: String,
    num_mines: usize,
    seed: u64,
}

impl MinesweeperGame {
    pub fn new(width: usize, height: usize, generator: Box<dyn Generator>, num_mines: usize, seed: u64) -> Self {
        Self {
            state: State::JustCreated,
            grid: MinesweeperGrid::empty(width, height),
            generator,
            status_message: String::new(),
            num_mines,
            seed
        }
    }

    pub fn reveal(&mut self, position: Position) {
        self.status_message = String::new();
        if let State::JustCreated = self.state {
            self.state = State::Playing;
            self.grid.update_content(
                self.generator.generate(
                    self.grid.width,
                    self.grid.height,
                    position,
                    self.num_mines,
                    self.seed,
                )
            );
        }
        if let State::Playing = self.state {
            let cell = self.grid.get(position);
            if cell.state == CellState::Covered {
            self.grid.reveal(position);
            } else {
                if let CellContent::Number(_) = cell.content {
                    self.grid.chord(position)
                }
            }
            if self.grid.count_revealed_mines() > 0 {
                self.grid.reveal_mines();
                self.state = State::GameOver;
            } else if self.grid.is_solved() {
                self.state = State::YouWon;
            }
        }
    }

    pub fn flag(&mut self, position: Position) {
        self.status_message = String::new();
        if self.state == State::JustCreated || self.state == State::Playing {
        self.grid.flag(position);
        }
    }
}
