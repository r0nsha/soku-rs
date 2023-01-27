#![forbid(unsafe_code)]

mod candidates;
mod cell;
mod consts;
mod digit;
mod error;
mod grid;

pub mod prelude {
    pub use crate::candidates::*;
    pub use crate::cell::*;
    pub use crate::consts::*;
    pub use crate::digit::*;
    pub use crate::error::*;
    pub use crate::grid::*;
}

// TODO: Sudoku
// TODO: Grid
// TODO: Given
// TODO: House
// TODO: Box
// TODO: Row
// TODO: Column
// TODO: Cell
// TODO: Candidate
// TODO: Digit
