use std::fmt::Display;

use rand::{thread_rng, Rng};

use crate::{consts::HOUSE_SIZE, sudoku::Sudoku};

pub(crate) fn latin_squares() -> Sudoku {
    println!("{}\n", LatinSquare::new());
    println!("{}\n", LatinSquare::new_empty());
    println!("{}\n", LatinSquare::new_cyclic());
    println!("{}\n", LatinSquare::new_random());
    todo!()
}

#[derive(Debug)]
struct LatinSquare([u8; HOUSE_SIZE]);

impl Display for LatinSquare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl LatinSquare {
    fn new() -> Self {
        todo!()
    }

    fn new_empty() -> Self {
        todo!()
    }

    fn new_cyclic() -> Self {
        todo!()
    }

    fn new_random() -> Self {
        todo!()
    }
}
