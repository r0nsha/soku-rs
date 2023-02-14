use soku::prelude::*;

fn main() -> SudokuResult<()> {
    let mut sudoku = Sudoku::new_unique();

    println!("{sudoku}");
    sudoku.solve_with(BruteForceSolver);
    println!("{sudoku}");

    Ok(())
}
