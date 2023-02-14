mod brute_force;

pub use brute_force::BruteForceSolver;

use crate::prelude::Sudoku;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Solution {
    Impossible,
    Unique,
    Ambiguous(usize),
}

impl Solution {
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        use Solution::{Ambiguous, Impossible, Unique};

        match (self, other) {
            (Impossible, s) | (s, Impossible) => s,
            (Unique, Unique) => Unique,
            (Unique, Ambiguous(x)) | (Ambiguous(x), Unique) => Ambiguous(x),
            (Ambiguous(x), Ambiguous(y)) => Ambiguous(x + y),
        }
    }

    #[must_use]
    pub const fn count(&self) -> usize {
        match self {
            Self::Impossible => 0,
            Self::Unique => 1,
            Self::Ambiguous(n) => *n,
        }
    }
}

pub trait Solve {
    fn solve(self, sudoku: &mut Sudoku) -> Solution;
}
