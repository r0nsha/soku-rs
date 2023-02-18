use crate::{
    measure,
    prelude::{Candidates, Sudoku},
};

use super::Solve;

// TODO: tests
// TODO: docs

// TODO: https://www.tutorialspoint.com/Sudoku-Solving-algorithms
pub struct BruteForceSolver;

impl Solve for BruteForceSolver {
    fn solve(self, sudoku: &mut Sudoku) -> bool {
        sudoku.fill_all_cell_candidates();
        // Self::solve_inner(sudoku)
        measure!("Solver", { Self::solve_inner(sudoku) })
    }
}

impl BruteForceSolver {
    fn solve_inner(sudoku: &mut Sudoku) -> bool {
        // TODO: whenever we set a cell, recalculate all candidates for its row, column and square
        // TODO: whenever we clear a cell, recalculate all candidates for its row, column and square
        if let Some((index, candidates)) = Self::cell_with_least_candidates(sudoku) {
            for digit in candidates.digits() {
                sudoku.set_cell(index, digit);

                if Self::solve_inner(sudoku) {
                    return true;
                }
            }

            sudoku.clear_cell(index);

            false
        } else {
            true
        }
    }

    fn cell_with_least_candidates(sudoku: &Sudoku) -> Option<(usize, Candidates)> {
        let mut best_candidates_count = usize::MAX;
        let mut result = None;

        for (i, _) in sudoku
            .cells()
            .enumerate()
            .filter(|(_, cell)| cell.digit.is_none())
        {
            // TODO: take candidates from the cell, don't recalculate them
            let candidates = sudoku.cell_candidates(i);
            let candidates_count = candidates.count();

            if candidates_count < best_candidates_count {
                best_candidates_count = candidates_count;
                result = Some((i, candidates));

                if best_candidates_count <= 1 {
                    break;
                }
            }
        }

        result
    }
}
