use crate::{prelude::{Candidates, Coord, Digit, Sudoku, SudokuIndex, HOUSE_INDICES}, measure};

use super::Solve;

// TODO: tests
// TODO: docs

pub struct BruteForceSolver;

impl BruteForceSolver {
    pub const fn new() -> Self {
        Self
    }
}

impl Solve for BruteForceSolver {
    fn solve(self, sudoku: &mut Sudoku) -> bool {
        let all_candidates = sudoku.all_candidates();
        Self::solve_inner(sudoku, all_candidates)
        // measure!("Solver", {
        //     let all_candidates = sudoku.all_candidates();
        //     Self::solve_inner(sudoku, all_candidates)
        // })
    }
}

impl BruteForceSolver {
    fn solve_inner(sudoku: &mut Sudoku, mut all_candidates: Vec<Candidates>) -> bool {
        if let Some((index, candidates)) = Self::cell_with_least_candidates(sudoku, &all_candidates)
        {
            for digit in candidates.digits() {
                sudoku.set_cell(index, digit);

                Self::recalculate_all_candidates(&mut all_candidates, index, digit);

                if Self::solve_inner(sudoku, all_candidates.clone()) {
                    return true;
                }
            }

            sudoku.clear_cell(index);

            false
        } else {
            true
        }
    }

    fn cell_with_least_candidates(
        sudoku: &Sudoku,
        all_candidates: &[Candidates],
    ) -> Option<(usize, Candidates)> {
        let mut best_candidates_count = usize::MAX;
        let mut result = None;

        for (i, _) in sudoku
            .cells()
            .enumerate()
            .filter(|(_, cell)| cell.digit.is_none())
        {
            // let candidates = sudoku.cell_candidates(i);
            let candidates = all_candidates[i];
            let candidates_count = candidates.count();

            if candidates_count < best_candidates_count {
                best_candidates_count = candidates_count;
                result = Some((i, candidates));

                if best_candidates_count <= 1 {
                    return result;
                }
            }
        }

        result
    }

    fn recalculate_all_candidates(all_candidates: &mut [Candidates], index: usize, digit: Digit) {
        let coord @ Coord(row, col) = Coord::from_index(index);

        // Remove digit from column
        for r in HOUSE_INDICES {
            if r != row {
                let index = Coord(r, col).into_index();
                all_candidates[index].remove(digit);
            }
        }

        // Remove digit from column
        for c in HOUSE_INDICES {
            if c != col {
                let index = Coord(row, c).into_index();
                all_candidates[index].remove(digit);
            }
        }

        for i in Sudoku::square_indices_of_cell(coord) {
            if i != index {
                all_candidates[i].remove(digit);
            }
        }
    }
}
