use crate::{CellContent, CellState, Generator, MinesweeperGrid};

enum State {
    JustCreated,
    Playing,
    GameOver,
}

pub struct Game {
    state: State,
    pub grid: MinesweeperGrid,
    generator: Box<dyn Generator>,
    // solver: Solver
}

impl Game {
    pub fn new(width: usize, height: usize, generator: impl Generator + 'static) -> Game {
        Game {
            state: State::JustCreated,
            grid: MinesweeperGrid::empty(width, height),
            generator: Box::new(generator),
        }
    }

    pub fn press_cell(&mut self, x: usize, y: usize) {
        if let State::JustCreated = self.state {
            self.state = State::Playing;
            self.grid.update_content(self.generator.generate(self.grid.width, self.grid.height, x, y));
        }
        if let State::Playing = self.state {
            let cell = self.grid.get(x, y);
            if cell.state == CellState::Covered {
            self.grid.reveal(x, y);
            } else {
                match cell.content {
                    CellContent::Number(_) => { self.grid.chord(x, y) },
                    _ => {}
                }
            }
        }
    }

    pub fn flag(&mut self, x: usize, y: usize) {
        self.grid.flag(x, y);
    }
}
