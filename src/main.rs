use soku::prelude::*;

fn main() -> Result<(), SudokuError> {
    let sudoku = Sudoku::new();
    println!("{sudoku}");
    Ok(())
}
