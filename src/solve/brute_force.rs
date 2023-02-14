use crate::prelude::{Coord, Sudoku, GRID_SIZE};

use super::{Constraint, Solution, Solve};

// TODO: tests
// TODO: docs

// TODO: https://www.tutorialspoint.com/Sudoku-Solving-algorithms
pub struct BruteForceSolver {
    pub constraint: Option<Constraint>,
}

impl Solve for BruteForceSolver {
    fn solve(self, sudoku: &mut Sudoku) -> Solution {
        self.solve_inner(sudoku, 0)
    }
}

impl BruteForceSolver {
    fn solve_inner(&self, sudoku: &mut Sudoku, mut index: usize) -> Solution {
        while index < GRID_SIZE && sudoku.cell(index).unwrap().digit.is_some() {
            index += 1;
        }

        if index == GRID_SIZE {
            return Solution::Unique;
        }

        let coord = Coord::from_index(index);
        let candidates = sudoku.cell_candidates(index);
        let candidate_count = candidates.count();

        for digit in candidates.digits() {
            match self.constraint {
                Some(constraint)
                    if candidate_count > 1 && constraint == Constraint(coord, digit) =>
                {
                    continue;
                }
                _ => (),
            }

            sudoku.set_cell(index, digit);

            if self.solve_inner(sudoku, index + 1) == Solution::Unique {
                return Solution::Unique;
            }
        }

        sudoku.clear_cell(index);

        Solution::Impossible
    }
}
