use std::{
    io::{Read, Write},
    time::Duration,
};

use crate::prelude::{Sudoku, GRID_SIZE};

use super::{Solution, Solve};

// TODO: tests
// TODO: docs

// TODO: https://www.tutorialspoint.com/Sudoku-Solving-algorithms
pub struct BruteForceSolver;

impl Solve for BruteForceSolver {
    fn solve(self, sudoku: &mut Sudoku) -> Solution {
        solve(sudoku, 0)
    }
}

fn solve(sudoku: &mut Sudoku, mut index: usize) -> Solution {
    while index < GRID_SIZE && sudoku.cell(index).unwrap().digit.is_some() {
        index += 1;
    }

    if index == GRID_SIZE {
        return Solution::Unique;
    }

    for digit in sudoku.cell_candidates(index).digits() {
        sudoku.set_cell(index, digit);
        println!("set index={index} to {digit}");
        println!("{sudoku}");
        pause();
        // std::thread::sleep(Duration::from_millis(100));

        if solve(sudoku, index + 1) == Solution::Unique {
            return Solution::Unique;
        }
    }

    dbg!(sudoku.cell_candidates(index));
    sudoku.clear_cell(index);
    println!("clear index={index}");
    println!("{sudoku}");
    pause();
    // std::thread::sleep(Duration::from_millis(100));

    Solution::Impossible
}

fn pause() {
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    stdout.flush().unwrap();
    let _ = stdin.read(&mut [0u8]).unwrap();
}
