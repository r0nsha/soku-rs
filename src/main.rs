use soku::prelude::*;

fn main() -> SudokuResult<()> {
    let mut sudoku = Sudoku::new_filled();
    let mut sudoku2 = sudoku.clone();

    // println!("{sudoku}");
    sudoku.solve_with(BruteForceSolver);
    println!("{sudoku}");

    // println!("{sudoku}");
    sudoku2.solve_with(ReverseBruteForceSolver);
    println!("{sudoku}");

    println!("{}", sudoku == sudoku2);

    Ok(())
}
