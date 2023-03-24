use soku::prelude::*;

fn main() -> SudokuResult<()> {
    let sudoku = Sudoku::new_unique(SudokuConfig { cells: 23 });
    println!("{sudoku}");
    Ok(())
}
