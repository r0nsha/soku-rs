use std::fmt::Display;

use derive_more::{Deref, DerefMut};
use rand::{seq::SliceRandom, thread_rng, Rng};

use crate::prelude::{Coord, Digit, Generate, Sudoku, DIGIT_INDICES, SQUARE_SIZE};

use super::{Config, Difficulty};

// TODO: tests
// TODO: docs

pub struct LatinSquares;

impl Generate for LatinSquares {
    fn generate(self, config: Config) -> Sudoku {
        let filled_sudoku = Self::generate_filled_sudoku();

        let mut rng = thread_rng();
        let target_filled_cells = match config.difficulty {
            Difficulty::Easy => 36,
            Difficulty::Medium => 30,
            Difficulty::Hard => 25,
            Difficulty::Expert => 23,
        };

        'pick_40_random_cells: loop {
            let mut sudoku = Self::with_40_random_cells(filled_sudoku.clone());

            let mut given_coords = sudoku
                .cells()
                .enumerate()
                .filter(|(_, cell)| cell.digit.is_some())
                .map(|(i, _)| Coord::from_index(i))
                .collect::<Vec<Coord>>();

            let mut loops = 0;

            'remove_givens: loop {
                loops += 1;

                if given_coords.is_empty() {
                    continue 'pick_40_random_cells;
                }

                let random_index = rng.gen_range(0..given_coords.len());
                let coord = given_coords[random_index];
                given_coords.swap_remove(random_index);

                let digit = sudoku.cell(coord).unwrap().digit.unwrap();

                sudoku.clear_cell(coord);

                if sudoku.is_unique() {
                    // TODO: use smarter difficulty rating: https://www.sudokuoftheday.com/difficulty
                    if sudoku.count_filled_cells() == target_filled_cells {
                        for cell in sudoku.cells_mut() {
                            cell.is_given = true;
                        }
                        println!("Generation took {loops} loops");
                        return sudoku;
                    } else {
                        continue 'remove_givens;
                    }
                } else {
                    sudoku.set_cell(coord, digit);
                }
            }
        }
    }
}

impl LatinSquares {
    fn generate_filled_sudoku() -> Sudoku {
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

        for (index, latin_square) in squares.iter().enumerate() {
            for (cell_index, cell) in sudoku.square_mut(index).enumerate() {
                let Coord(row, col) = Coord::from_index_of(cell_index, SQUARE_SIZE);

                let latin_square_digit = latin_square[row][col];
                let digit = Digit::new_unchecked(latin_square_digit);

                cell.digit = Some(digit);
            }
        }

        Self::swap_rows(&mut sudoku);

        sudoku
    }

    fn swap_rows(sudoku: &mut Sudoku) {
        fn inner(sudoku: &mut Sudoku, r1: usize, r2: usize) {
            for col in DIGIT_INDICES {
                let r1_coord = Coord(r1, col);
                let r2_coord = Coord(r2, col);

                let temp_digit = sudoku.cell(r1_coord).unwrap().digit;
                sudoku.cell_mut(r1_coord).unwrap().digit = sudoku.cell(r2_coord).unwrap().digit;
                sudoku.cell_mut(r2_coord).unwrap().digit = temp_digit;
            }
        }

        inner(sudoku, 1, 3);
        inner(sudoku, 2, 6);
        inner(sudoku, 5, 7);
    }

    fn with_40_random_cells(sudoku: Sudoku) -> Sudoku {
        fn inner(mut sudoku: Sudoku) -> Sudoku {
            const TO_REMOVE: usize = 40;

            let mut rng = thread_rng();
            let mut removed = 0;

            while removed < TO_REMOVE {
                let random_coord = Coord::random(&mut rng);

                let cell = sudoku.cell_mut(random_coord).expect("cell to exist");

                if cell.digit.is_some() {
                    cell.digit = None;
                    removed += 1;
                }
            }

            sudoku
        }

        loop {
            let sudoku = inner(sudoku.clone());

            if sudoku.is_unique() {
                break sudoku;
            }
        }
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
