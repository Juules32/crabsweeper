use crate::{Generator, MinesweeperGrid};

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

    pub fn reveal(&mut self, x: usize, y: usize) {
        if let State::JustCreated = self.state {
            self.state = State::Playing;
            self.grid = self.generator.generate(self.grid.width, self.grid.height);
        }
        if let State::Playing = self.state {
            self.grid.reveal(x, y);
        }
    }

    pub fn flag(&mut self, x: usize, y: usize) {
        self.grid.flag(x, y);
    }
}
