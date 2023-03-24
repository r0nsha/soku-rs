mod latin_squares;

use crate::prelude::Sudoku;
pub use latin_squares::LatinSquares;

pub trait Generate {
    fn generate(self, config: Config) -> Sudoku;
    fn generate_filled(self, config: Config) -> Sudoku;
    fn generate_from(self, sudoku: Sudoku, config: Config) -> Sudoku;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    pub cells: usize,
}
