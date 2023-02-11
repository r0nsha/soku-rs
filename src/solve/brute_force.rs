use crate::prelude::{Sudoku, GRID_SIZE};

use super::{Solution, Solve};

// TODO: tests
// TODO: docs

// TODO: https://www.tutorialspoint.com/Sudoku-Solving-algorithms
pub struct BruteForceSolver;

impl Solve for BruteForceSolver {
    fn solve(self, sudoku: &mut Sudoku) -> Solution {
        solve(sudoku, 0)
    }
}

fn solve(sudoku: &mut Sudoku, mut index: usize) -> Solution {
    while index < GRID_SIZE && sudoku.cell(index).unwrap().digit.is_some() {
        index += 1;
    }

    if index == GRID_SIZE {
        return Solution::Unique;
    }

    for digit in sudoku.cell_candidates(index).digits() {
        sudoku.set_cell(index, digit);

        if solve(sudoku, index + 1) == Solution::Unique {
            return Solution::Unique;
        }
    }

    sudoku.clear_cell(index);

    Solution::Impossible
}
