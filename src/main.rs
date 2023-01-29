use soku::prelude::*;

fn main() -> Result<(), SudokuError> {
    let sudoku = Sudoku::new_filled();
    println!("{sudoku}");
    Ok(())
}
