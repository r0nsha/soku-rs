#![forbid(unsafe_code)]
#![deny(
    clippy::correctness,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
#![allow(clippy::module_name_repetitions)]

mod consts;
mod generate;
mod sudoku;

pub mod prelude {
    pub use crate::consts::*;
    pub use crate::generate::*;
    pub use crate::sudoku::*;
}
