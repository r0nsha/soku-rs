use soku::prelude::*;

fn main() -> SudokuResult<()> {
    let mut sudoku = Sudoku::new_filled();
    println!("{sudoku}");
    println!("{}", sudoku.is_valid());
    sudoku.solve_with(BruteForceSolver);
    println!("{sudoku}");
    println!("{}", sudoku.is_valid());
    Ok(())
}
