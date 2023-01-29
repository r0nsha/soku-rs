use std::{
    fmt::{Display, Write},
    ops::Deref,
    slice::Chunks,
};

use derive_more::Deref;

use crate::{
    consts::{GRID_SIZE, HOUSE_SIZE, SQUARE_SIZE},
    error::SudokuError,
    generate,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Sudoku(pub(crate) [Cell; GRID_SIZE]);

impl Default for Sudoku {
    fn default() -> Self {
        Self([Default::default(); GRID_SIZE])
    }
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const PIPE: char = '|';

        f.write_str(".-----.-----.-----.\n")?;

        for (row_index, row) in self.rows().enumerate() {
            f.write_char(PIPE)?;

            for (cell_index, cell) in row.iter().enumerate() {
                f.write_char(cell.digit.map_or('.', |d| *d as char))?;

                if (cell_index + 1) % 3 == 0 {
                    f.write_char(PIPE)?;
                } else {
                    f.write_char(' ')?;
                }
            }

            f.write_char('\n')?;

            if row_index < HOUSE_SIZE - 1 && (row_index + 1) % 3 == 0 {
                f.write_str(":----- ----- -----:\n")?;
            }
        }

        f.write_str("'-----'-----'-----'")?;

        Ok(())
    }
}

impl Sudoku {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn filled() -> Self {
        generate::latin_squares()
    }

    pub fn rows(&self) -> Chunks<'_, Cell> {
        self.0.chunks(HOUSE_SIZE)
    }

    pub fn squares(&self) -> Squares<'_> {
        Squares {
            sudoku: self,
            index: 0,
        }
    }
}

pub struct Squares<'a> {
    sudoku: &'a Sudoku,
    index: usize,
}

impl<'a> Iterator for Squares<'a> {
    type Item = [&'a Cell; HOUSE_SIZE];

    fn next(&mut self) -> Option<Self::Item> {
        if (self.index + HOUSE_SIZE * 2) >= self.sudoku.0.len() {
            return None;
        }

        let mut square: Self::Item = [&self.sudoku.0[0]; HOUSE_SIZE];
        let mut square_index = 0;
        let mut sudoku_index = self.index;

        for _ in 0..SQUARE_SIZE {
            println!(
                "square index: {square_index}, sudoku: {},{}",
                sudoku_index / HOUSE_SIZE,
                sudoku_index % HOUSE_SIZE,
            );
            square[square_index] = &self.sudoku.0[sudoku_index];
            square_index += 1;
            sudoku_index += 1;
        }

        sudoku_index += HOUSE_SIZE;

        for _ in 0..SQUARE_SIZE {
            square[square_index] = &self.sudoku.0[sudoku_index];
            square_index += 1;
            sudoku_index += 1;
        }

        sudoku_index += HOUSE_SIZE;

        for _ in 0..SQUARE_SIZE {
            square[square_index] = &self.sudoku.0[sudoku_index];
            square_index += 1;
            sudoku_index += 1;
        }

        if self.index % HOUSE_SIZE == 0 {
            self.index += HOUSE_SIZE * 2;
        } else {
            self.index += SQUARE_SIZE;
        }

        Some(square)

        // 0, 1, 2
        // 9, 10, 11
        //
        // 3, 4, 5
        //
        // 6, 7, 8
    }
}

// #[derive(Debug,Default,PartialEq, Eq)]
// pub struct Candidates(BitSet<u8>);

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Cell {
    pub(crate) digit: Option<Digit>,
    //pub(crate) candidates: Candidates,
}

#[derive(Debug, Default, Deref, PartialEq, Eq, Clone, Copy)]
pub struct Digit(pub(crate) u8);

impl Digit {
    #[inline]
    pub fn is_valid(value: u8) -> bool {
        (1..=9).contains(&value)
    }

    #[inline]
    pub fn new(value: u8) -> Result<Self, SudokuError> {
        Self::try_from(value)
    }

    #[inline]
    pub fn new_unchecked(value: u8) -> Self {
        Self(value)
    }
}

impl TryFrom<u8> for Digit {
    type Error = SudokuError;

    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if Digit::is_valid(value) {
            Ok(Self(value))
        } else {
            Err(SudokuError::InvalidDigit(value))
        }
    }
}
