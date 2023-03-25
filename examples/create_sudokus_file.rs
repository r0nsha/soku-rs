use std::{
    fs,
    io::Write,
    sync::atomic::{AtomicUsize, Ordering},
};

use futures::{future, stream::FuturesUnordered};
use soku::{measure, prelude::*};

static PROGRESS: AtomicUsize = AtomicUsize::new(0);
const TOTAL: usize = 1000;
const CELLS: usize = 17;

#[tokio::main]
async fn main() {
    measure!("Generate sudokus file", {
        let futures = (0..TOTAL)
            .map(|_| {
                tokio::spawn(async {
                    let result = Sudoku::new_unique(SudokuConfig { cells: CELLS }).to_str_line();
                    PROGRESS.fetch_add(1, Ordering::Relaxed);
                    println!("progress: {PROGRESS:?}/{TOTAL}");
                    result
                })
            })
            .collect::<FuturesUnordered<_>>();

        let results = future::join_all(futures).await;

        let mut file = fs::File::create("sudokus.txt").unwrap();

        for result in results {
            let str = result.unwrap();
            // file.write_all(str.as_bytes()).unwrap();
            writeln!(file, "{}", str).unwrap();
        }
    });
}
