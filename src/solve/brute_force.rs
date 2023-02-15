use crate::prelude::{Sudoku, GRID_SIZE};

use super::Solve;

// TODO: tests
// TODO: docs

// TODO: https://www.tutorialspoint.com/Sudoku-Solving-algorithms
pub struct BruteForceSolver;

impl Solve for BruteForceSolver {
    fn solve(self, sudoku: &mut Sudoku) -> bool {
        let now = std::time::Instant::now();
        let result = Self::solve_inner(sudoku, 0);
        println!("Elapsed: {:.2?}", now.elapsed());
        result
    }
}

impl BruteForceSolver {
    fn solve_inner(sudoku: &mut Sudoku, mut index: usize) -> bool {
        while index < GRID_SIZE && sudoku.cell(index).unwrap().digit.is_some() {
            index += 1;
        }

        if index == GRID_SIZE {
            return true;
        }

        for digit in sudoku.cell_candidates(index).digits() {
            sudoku.set_cell(index, digit);

            if Self::solve_inner(sudoku, index + 1) {
                return true;
            }
        }

        sudoku.clear_cell(index);

        false
    }
}
