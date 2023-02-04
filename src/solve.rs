mod backtracking;

pub use backtracking::BacktrackingSolver;

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
            (Impossible, _) | (_, Impossible) => Impossible,
            (Unique, Unique) => Unique,
            (Unique, Ambiguous(x)) | (Ambiguous(x), Unique) => Ambiguous(x),
            (Ambiguous(x), Ambiguous(y)) => Ambiguous(x + y),
        }
    }
}

pub trait Solve {
    fn solve(self, sudoku: &mut Sudoku);
}
