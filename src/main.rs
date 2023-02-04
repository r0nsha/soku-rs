use soku::prelude::*;

fn main() -> SudokuResult<()> {
    let mut sudoku = Sudoku::new_filled();
    println!("{sudoku}");
    Ok(())
}
