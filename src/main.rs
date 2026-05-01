use crabsweeper::{Presentation, window_conf};

#[macroquad::main(window_conf)]
async fn main() {
    Presentation::run().await;
}
