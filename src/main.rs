use soku::prelude::*;

fn main() -> SudokuResult<()> {
    let sudoku = Sudoku::new_unique();
    Ok(())
}
