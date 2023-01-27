#![forbid(unsafe_code)]

mod consts;
mod error;
mod generate;
mod sudoku;

pub mod prelude {
    pub use crate::consts::*;
    pub use crate::error::*;
    pub use crate::sudoku::*;
}
