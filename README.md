# Crabsweeper

## How to build for web
- Add nightly: `rustup toolchain install nightly`
- Use nightly: `rustup override set nightly`
- Add web target: `rustup target add wasm32-unknown-unknown`
- Build: `cargo build --target wasm32-unknown-unknown --release`
- Copy `target/wasm32-unknown-unknown/release/crabsweeper.wasm` to the `web` folder
- Run a local web server with something like [live-server](https://www.npmjs.com/package/live-server)
