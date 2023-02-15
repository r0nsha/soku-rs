use soku::prelude::*;

fn main() -> SudokuResult<()> {
    let now = std::time::Instant::now();
    let sudoku = Sudoku::new_unique(Config {
        difficulty: Difficulty::Hard,
    });
    println!("Elapsed: {:.2?}", now.elapsed());
    println!("{sudoku}");
    Ok(())
}
