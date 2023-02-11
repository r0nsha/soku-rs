use crate::prelude::{Sudoku, GRID_SIZE};

use super::{Solution, Solve};

// TODO: tests
// TODO: docs

// TODO: https://www.tutorialspoint.com/Sudoku-Solving-algorithms
pub struct BruteForceSolver;

impl Solve for BruteForceSolver {
    fn solve(self, sudoku: &mut Sudoku) -> Solution {
        Self::solve_inner(sudoku, 0)
    }
}

impl BruteForceSolver {
    fn solve_inner(sudoku: &mut Sudoku, mut index: usize) -> Solution {
        while index < GRID_SIZE && sudoku.cell(index).unwrap().digit.is_some() {
            index += 1;
        }

        if index == GRID_SIZE {
            return Solution::Unique;
        }

        for digit in sudoku.cell_candidates(index).digits() {
            sudoku.set_cell(index, digit);

            if Self::solve_inner(sudoku, index + 1) == Solution::Unique {
                return Solution::Unique;
            }
        }

        sudoku.clear_cell(index);

        Solution::Impossible
    }
}

// Mainly used for checking whether there's more than one solution for a given sudoku
pub struct ReverseBruteForceSolver;

impl Solve for ReverseBruteForceSolver {
    fn solve(self, sudoku: &mut Sudoku) -> Solution {
        Self::solve_inner(sudoku, 80)
    }
}

impl ReverseBruteForceSolver {
    fn solve_inner(sudoku: &mut Sudoku, mut index: usize) -> Solution {
        while index > 0 && sudoku.cell(index).unwrap().digit.is_some() {
            index -= 1;
        }

        if index == GRID_SIZE {
            return Solution::Unique;
        }

        for digit in sudoku.cell_candidates(index).digits() {
            sudoku.set_cell(index, digit);

            if index > 0 && Self::solve_inner(sudoku, index - 1) == Solution::Unique {
                return Solution::Unique;
            }
        }

        sudoku.clear_cell(index);

        Solution::Impossible
    }
}
