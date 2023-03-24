mod latin_squares;

use crate::prelude::Sudoku;
pub use latin_squares::LatinSquares;

pub trait Generate {
    fn generate(self, config: SudokuConfig) -> Sudoku;
    fn generate_filled(self, config: SudokuConfig) -> Sudoku;
    fn generate_from(self, sudoku: Sudoku, config: SudokuConfig) -> Sudoku;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SudokuConfig {
    pub cells: usize,
}
