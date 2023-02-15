use crate::prelude::{Candidates, Sudoku};

use super::Solve;

// TODO: tests
// TODO: docs

// TODO: https://www.tutorialspoint.com/Sudoku-Solving-algorithms
pub struct BruteForceSolver;

impl Solve for BruteForceSolver {
    fn solve(self, sudoku: &mut Sudoku) -> bool {
        // let now = std::time::Instant::now();
        let result = Self::solve_inner(sudoku);
        // println!("Elapsed: {:.2?}", now.elapsed());
        result
    }
}

impl BruteForceSolver {
    fn solve_inner(sudoku: &mut Sudoku) -> bool {
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

    // TODO: needs a refactor
    fn cell_with_least_candidates(sudoku: &Sudoku) -> Option<(usize, Candidates)> {
        let mut best_candidates_count = usize::MAX;
        let mut result = None;

        for (i, cell) in sudoku.cells().enumerate() {
            if cell.digit.is_none() {
                let candidates = sudoku.cell_candidates(i);
                let candidates_count = candidates.count();

                if candidates_count < best_candidates_count {
                    best_candidates_count = candidates_count;
                    result = Some((i, candidates));

                    if best_candidates_count == 0 {
                        break;
                    }
                }
            }
        }

        result
    }
}
