#![forbid(unsafe_code)]
// TODO: #![forbid(missing_docs)]

mod consts;
mod error;
mod generate;
mod sudoku;

pub mod prelude {
    pub use crate::consts::*;
    pub use crate::error::*;
    pub use crate::sudoku::*;
}
