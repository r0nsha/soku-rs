use soku::prelude::*;

fn main() -> Result<(), SudokuError> {
    let sudoku = Sudoku::filled();
    println!("{sudoku}");
    Ok(())
}
