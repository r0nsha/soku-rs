use std::fmt::Display;

use derive_more::{Deref, DerefMut};
use rand::{seq::SliceRandom, thread_rng};

use crate::prelude::{Coord, Digit, Generate, Sudoku, DIGIT_INDICES, SQUARE_SIZE};

pub struct LatinSquares;

impl Generate for LatinSquares {
    fn generate(self) -> Sudoku {
        const fn convert_base_3_to_10(mut num: u8) -> u8 {
            let mut i = 0u8;
            let mut result = 0;

            while num > 0 {
                let exponent = 3u8.pow(i as _);
                result += (num % 10) * exponent;
                num /= 10;
                i += 1;
            }

            result
        }

        fn swap_rows(sudoku: &mut Sudoku, r1: usize, r2: usize) {
            for col in DIGIT_INDICES {
                let r1_coord = Coord(r1, col);
                let r2_coord = Coord(r2, col);

                let temp_digit = sudoku.cell(r1_coord).unwrap().digit;
                sudoku.cell_mut(r1_coord).unwrap().digit = sudoku.cell(r2_coord).unwrap().digit;
                sudoku.cell_mut(r2_coord).unwrap().digit = temp_digit;
            }
        }

        // We follow the algorithm from this paper: https://sites.math.washington.edu/~morrow/mcm/team2280.pdf
        // Select nine 3x3 random latin squares
        let mut squares = std::iter::repeat_with(LatinSquare::new_random)
            .take(9)
            .collect::<Vec<_>>();

        // Create another latin square, which corresponds to each square in the previous vec
        let big_square = LatinSquare::new_random();

        // For each digit in each square, pair it with the digit of the corresponding big_square digit.
        // Treat this paired number as base 3 and convert to base 10, adding 1
        let mut row = 0;
        let mut col = 0;

        for square in &mut squares {
            let big_square_value = big_square[row][col] * 10;

            for row in square.iter_mut() {
                for digit in row.iter_mut() {
                    // pair the digit with the big_square value
                    let paired = *digit + big_square_value;

                    // convert the paired numbers from base 3 to base 10
                    let base_10 = convert_base_3_to_10(paired);

                    *digit = base_10 + 1;
                }
            }

            if col == 2 {
                col = 0;
                row += 1;
            } else {
                col += 1;
            }
        }

        // Fill a sudoku board with the generated squares
        let mut sudoku = Sudoku::new_empty();

        for (idx, latin_square) in squares.iter().enumerate() {
            for (cell_idx, cell) in sudoku.square_mut_by_index(idx).enumerate() {
                let Coord(row, col) = Coord::from_index(cell_idx, SQUARE_SIZE);

                let latin_square_digit = latin_square[row][col];
                let digit = Digit::new_unchecked(latin_square_digit);

                cell.digit = Some(digit);
            }
        }

        swap_rows(&mut sudoku, 1, 3);
        swap_rows(&mut sudoku, 2, 6);
        swap_rows(&mut sudoku, 5, 7);

        sudoku
    }
}

#[derive(Debug, Default, Deref, DerefMut)]
struct LatinSquare([[u8; SQUARE_SIZE]; SQUARE_SIZE]);

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
    fn new_random() -> Self {
        let mut rng = thread_rng();
        let mut square = Self::default();

        let size = square.len();

        // Initialize square
        for row in square.iter_mut() {
            for (i, cell) in row.iter_mut().enumerate() {
                *cell = i.try_into().unwrap();
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
            let mut used = [false; SQUARE_SIZE];

            for i in 0..size - 1 {
                used[square[i][j] as usize] = true;
            }

            for (i, used) in used.iter().enumerate() {
                if !used {
                    square[size - 1][j] = i.try_into().unwrap();
                    break;
                }
            }
        }

        square
    }
}
