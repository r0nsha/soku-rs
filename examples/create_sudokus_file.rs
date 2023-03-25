use std::{
    fs,
    sync::atomic::{AtomicUsize, Ordering},
};

use futures::{future, stream::FuturesUnordered};
use serde_json::json;
use soku::{measure, prelude::*};

static PROGRESS: AtomicUsize = AtomicUsize::new(0);
const TOTAL: usize = 1000;
const CELLS: usize = 20;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    measure!("Generate sudokus file", {
        let futures = (0..TOTAL)
            .map(|_| {
                tokio::spawn(async {
                    let config = SudokuConfig { cells: CELLS };
                    let filled = Sudoku::new_filled(config);
                    let sudoku = LatinSquares.generate_from(filled.clone(), config);
                    // let result = Sudoku::new_unique(SudokuConfig { cells: CELLS }).to_str_line();
                    PROGRESS.fetch_add(1, Ordering::Relaxed);
                    println!("progress: {PROGRESS:?}/{TOTAL}");
                    (filled, sudoku)
                })
            })
            .collect::<FuturesUnordered<_>>();

        let results = future::join_all(futures).await;

        let jsons = results
            .into_iter()
            .map(|result| {
                let (filled, sudoku) = result.unwrap();
                let (filled, sudoku) = (filled.to_str_line(), sudoku.to_str_line());

                json!({
                    "filled": filled,
                    "sudoku": sudoku
                })
            })
            .collect::<Vec<_>>();

        let entire_json = serde_json::to_string_pretty(&jsons).unwrap();

        fs::write("sudokus.json", entire_json).unwrap();
    });
}
