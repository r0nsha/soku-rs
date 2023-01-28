use std::fmt::Display;

use derive_more::{Deref, DerefMut};
use rand::{seq::SliceRandom, thread_rng, Rng};

use crate::{
    consts::{BOX_SIZE, HOUSE_SIZE},
    sudoku::Sudoku,
};

pub(crate) fn latin_squares() -> Sudoku {
    println!("{}\n", LatinSquare::new());
    todo!()
}

#[derive(Debug, Default, Deref, DerefMut)]
struct LatinSquare([[u8; BOX_SIZE]; BOX_SIZE]);

impl Display for LatinSquare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.iter() {
            for cell in row.iter() {
                write!(f, "{cell}")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl LatinSquare {
    fn new() -> Self {
        let mut rng = thread_rng();
        let mut square = Self::default();

        let size = square.len();

        // Initialize square
        for row in square.iter_mut() {
            for (index, cell) in row.iter_mut().enumerate() {
                *cell = index as u8;
            }
        }

        // First row
        square[0].shuffle(&mut rng);

        // Middle row(s)
        for i in 1..size - 1 {
            'shuffling: loop {
                square[i].shuffle(&mut rng);

                for k in 0..i {
                    for j in 0..size {
                        if square[k][j] == square[i][j] {
                            continue 'shuffling;
                        }
                    }
                }

                break 'shuffling;
            }
        }

        // Last row
        for j in 0..size {
            let mut used = [false; BOX_SIZE];

            for i in 0..size - 1 {
                used[square[i][j] as usize] = true;
            }

            for (index, used) in used.iter().enumerate() {
                if !used {
                    square[size - 1][j] = index as u8;
                    break;
                }
            }
        }

        square
    }
}
