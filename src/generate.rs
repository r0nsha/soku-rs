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
    pub difficulty: Difficulty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    VeryHard,
    Expert,
    Insane,
}

impl Difficulty {
    pub const fn into_cell_count(self) -> usize {
        match self {
            Self::Easy => 62,
            Self::Medium => 53,
            Self::Hard => 44,
            Self::VeryHard => 35,
            Self::Expert => 26,
            Self::Insane => 17,
        }
    }
}
