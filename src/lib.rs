#![forbid(unsafe_code)]

mod consts;
mod error;
mod sudoku;

pub mod prelude {
    pub use crate::consts::*;
    pub use crate::error::*;
    pub use crate::sudoku::*;
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
