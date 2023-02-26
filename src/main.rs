use soku::{measure, prelude::*};

fn main() -> SudokuResult<()> {
    let sudoku = measure!("Total", {
        Sudoku::new_unique(Config {
            difficulty: Difficulty::Insane,
        })
    });
    println!("{sudoku}");
    Ok(())
}
