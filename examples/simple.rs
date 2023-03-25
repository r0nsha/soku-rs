use soku::{measure, prelude::*};

fn main() -> SudokuResult<()> {
    let sudoku = measure!("Generation", {
        Sudoku::new_unique(SudokuConfig { cells: 23 })
    });
    println!("{sudoku}");
    Ok(())
}
