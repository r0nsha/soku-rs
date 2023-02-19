use crate::prelude::{Candidates, Coord, Digit, Sudoku, SudokuIndex, HOUSE_INDICES};

use super::Solve;

// TODO: tests
// TODO: docs

pub struct BruteForceSolver;

impl Solve for BruteForceSolver {
    fn solve(self, sudoku: &mut Sudoku) -> bool {
        Self::solve_inner(sudoku)
        // measure!("Solver", { Self::solve_inner(sudoku,) })
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

    fn cell_with_least_candidates(sudoku: &Sudoku) -> Option<(usize, Candidates)> {
        let mut best_candidates_count = usize::MAX;
        let mut result = None;

        for (i, _) in sudoku
            .cells()
            .enumerate()
            .filter(|(_, cell)| cell.digit.is_none())
        {
            let candidates = sudoku.cell_candidates(i);
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

    #[allow(dead_code)]
    fn get_unique_candidate(
        all_candidates: &[Candidates],
        cell_index: usize,
        cell_candidates: Candidates,
    ) -> Option<Digit> {
        let cell_coord = Coord::from_index(cell_index);

        fn unique_in_house(
            mut house_indices: impl Iterator<Item = usize>,
            all_candidates: &[Candidates],
            cell_index: usize,
            digit: Digit,
        ) -> bool {
            !house_indices.any(|index| index != cell_index && all_candidates[index].contains(digit))
        }

        for digit in cell_candidates.digits() {
            // Digit is unique in its row
            if unique_in_house(
                HOUSE_INDICES.map(|i| Coord(cell_coord.row(), i).into_index()),
                all_candidates,
                cell_index,
                digit,
            ) {
                return Some(digit);
            }

            // Digit is unique in its column
            if unique_in_house(
                HOUSE_INDICES.map(|i| Coord(i, cell_coord.col()).into_index()),
                all_candidates,
                cell_index,
                digit,
            ) {
                return Some(digit);
            }

            // Digit is unique in its square
            if unique_in_house(
                Sudoku::square_indices_of_cell(cell_coord).iter().copied(),
                all_candidates,
                cell_index,
                digit,
            ) {
                return Some(digit);
            }
        }

        None
    }
}
