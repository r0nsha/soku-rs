use soku::prelude::*;

fn main() -> SudokuResult<()> {
    let sudoku = Sudoku::new_unique(Config {
        difficulty: Difficulty::Easy,
    });
    println!("{sudoku}");
    Ok(())
}
