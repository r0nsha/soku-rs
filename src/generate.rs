use std::fmt::Display;

use derive_more::{Deref, DerefMut};
use rand::{seq::SliceRandom, thread_rng};

use crate::{
    consts::{BOX_SIZE, HOUSE_SIZE},
    sudoku::Sudoku,
};

pub(crate) fn latin_squares() -> Sudoku {
    // We follow the algorithm from this paper
    let mut squares = std::iter::repeat_with(LatinSquare::new)
        .take(9)
        .collect::<Vec<_>>();

    let big_square = LatinSquare::new();

    let mut x = 0;
    let mut y = 0;

    for (square_index, square) in squares.iter_mut().enumerate() {
        println!("{square}");

        let big_square_value = big_square[x][y] * 10;

        for row in square.iter_mut() {
            for digit in row.iter_mut() {
                // pair the digit with the big_square value
                let paired = *digit + big_square_value;

                // convert from base 3 to base 10
                let base_10 = { todo!() };
            }
        }

        println!("{square}");

        if y == 2 {
            y = 0;
            x += 1;
        } else {
            y += 1;
        }
    }

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
            for (i, cell) in row.iter_mut().enumerate() {
                *cell = i as u8;
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

            for (i, used) in used.iter().enumerate() {
                if !used {
                    square[size - 1][j] = i as u8;
                    break;
                }
            }
        }

        square
    }
}
