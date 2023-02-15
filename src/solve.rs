mod brute_force;

pub use brute_force::BruteForceSolver;

use crate::prelude::Sudoku;

pub trait Solve {
    fn solve(self, sudoku: &mut Sudoku) -> bool;
}
