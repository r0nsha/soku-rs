use soku::{measure, prelude::*};

fn main() -> SudokuResult<()> {
    measure!("Total", {
        let sudoku = Sudoku::new_unique(Config {
            difficulty: Difficulty::Hard,
        });
        println!("{sudoku}");
    });

    Ok(())
}
