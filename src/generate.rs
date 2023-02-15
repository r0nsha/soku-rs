mod latin_squares;

use crate::prelude::Sudoku;
pub use latin_squares::LatinSquares;

pub trait Generate {
    fn generate(self, config: Config) -> Sudoku;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    pub difficulty: Difficulty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Expert,
}
