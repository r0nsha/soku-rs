use crate::prelude::{Coord, Sudoku};

use super::Solve;

// TODO: tests
// TODO: docs

// TODO: https://www.tutorialspoint.com/Sudoku-Solving-algorithms
pub struct BacktrackingSolver {
    // current: Coord,
}

impl Solve for BacktrackingSolver {
    fn solve(self, sudoku: &mut Sudoku) {
        // Begin
        //  if no place in the grid is empty, then
        //   return true
        //  for number 1 to 9, do
        //   if isValidPlace(row, col, number), then
        //       grid[row, col] := number
        //       if solveSudoku = true, then
        //          return true
        //       grid[row, col] := 0

        //  done
        //  return false
        // End
    }
}

impl BacktrackingSolver {}
