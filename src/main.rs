use soku::{measure, prelude::*};

fn main() -> SudokuResult<()> {
    let sudoku = measure!("Total", { Sudoku::new_unique(SudokuConfig { cells: 22 }) });
    println!("{sudoku}");
    Ok(())
}
