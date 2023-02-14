use soku::prelude::*;

fn main() -> SudokuResult<()> {
    loop {
        let sudoku = Sudoku::new_unique();
        if sudoku.is_unique() {
            println!("yay!!")
        }
    }
    Ok(())
}
