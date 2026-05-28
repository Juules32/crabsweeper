# Crabsweeper
This is a project written in Rust with the purpose of demonstrating and measuring the performance of a minesweeper implementation that is guaranteed to be solvable.

The application was developed as part of the Prodecural Content Generation for Games course at the IT University of Copenhagen in 2026.

## How to run
>[!NOTE]
> You don't have to build the application yourself! Check out [releases](https://github.com/Juules32/crabsweeper/releases).

- Add nightly: `rustup toolchain install nightly`
- Use nightly: `rustup override set nightly`
- Run: `cargo run --release --bin crabsweeper`
- Run a performance test: `cargo run --release --bin performance_test`

## How to build for web
- Add nightly: `rustup toolchain install nightly`
- Use nightly: `rustup override set nightly`
- Add web target: `rustup target add wasm32-unknown-unknown`
- Build: `cargo build --release --target wasm32-unknown-unknown --bin crabsweeper`
- Copy `target/wasm32-unknown-unknown/release/crabsweeper.wasm` to the `web` folder
- Run a local web server with something like [live-server](https://www.npmjs.com/package/live-server)
