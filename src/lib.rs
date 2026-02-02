#![deny(unsafe_op_in_unsafe_fn)]
extern crate core;

mod grid;
mod cell;

use cell::{CellContent, CellState, Cell};
pub use grid::{Grid, Bitmap};
