use crate::prelude::{Cell, Coord, Digit, Sudoku, GRID_SIZE};

use super::{Solution, Solve};

// TODO: tests
// TODO: docs

// TODO: https://www.tutorialspoint.com/Sudoku-Solving-algorithms
pub struct BruteForceSolver;

impl Solve for BruteForceSolver {
    fn solve(self, sudoku: &mut Sudoku) -> Solution {
        if sudoku.is_filled() {
            return Solution::Unique;
        }

        Solution::Impossible
    }
}

fn solve(sudoku: &mut Sudoku, mut index: usize) -> bool {
    while index < GRID_SIZE && matches!(sudoku.cell(index), None) {
        index += 1;
    }

    if index == GRID_SIZE {
        return true;
    }

    false
}

fn is_valid_cell(sudoku: &Sudoku, row: usize, col: usize, digit: Digit) -> bool {
    fn house_contains<'a>(mut house: impl Iterator<Item = &'a Cell>, digit: Digit) -> bool {
        house.any(|cell| cell.digit == Some(digit))
    }

    let row_iter = sudoku.row(row);
    let col_iter = sudoku.col(col);
    let square_iter = sudoku.square(Coord(row - row % 3, col - col % 3));

    !house_contains(row_iter, digit)
        && !house_contains(col_iter, digit)
        && !house_contains(square_iter, digit)
}
