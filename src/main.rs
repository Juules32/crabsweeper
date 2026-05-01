use crabsweeper::{MinesweeperGame, RandomGenerator, Presentation, hash, window_conf};

#[macroquad::main(window_conf)]
async fn main() {
    let game = MinesweeperGame::new(9, 9, Box::new(RandomGenerator { seed: hash(&String::new()), num_mines: 10 }));
    let mut presentation = Presentation::new(game).await;
    presentation.run().await;
}
