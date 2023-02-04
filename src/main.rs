use soku::prelude::*;

fn main() -> SudokuResult<()> {
    let sudoku = Sudoku::new_filled();
    println!("{sudoku}");
    Ok(())
}
