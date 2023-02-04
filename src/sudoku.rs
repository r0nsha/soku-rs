use std::{
    fmt::{Display, Write},
    hash::Hash,
    slice::Chunks,
};

use derive_more::{Deref, Display};
use itertools::Itertools;
use thiserror::Error;

use crate::prelude::{
    Generate, LatinSquares, DIGITS, DIGIT_INDICES, GRID_SIZE, HOUSE_SIZE, SQUARE_SIZE,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Sudoku(pub(crate) [Cell; GRID_SIZE]);

impl Sudoku {
    #[must_use]
    pub fn new_empty() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn new_filled() -> Self {
        Self::new_with_generator(LatinSquares)
    }

    #[must_use]
    pub fn new_with_generator(generator: impl Generate) -> Self {
        generator.generate()
    }

    // TODO: from_str
    // TODO: from_str_lines

    #[inline]
    #[must_use]
    pub fn cell(&self, coord: Coord) -> Option<&Cell> {
        self.0.get(coord.as_index())
    }

    #[inline]
    pub fn cell_mut(&mut self, coord: Coord) -> Option<&mut Cell> {
        self.0.get_mut(coord.as_index())
    }

    pub fn row(&self, idx: usize) -> impl Iterator<Item = &'_ Cell> {
        Self::assert_house_index(idx);
        self.0.iter().skip(idx * HOUSE_SIZE).take(HOUSE_SIZE)
    }

    pub fn row_mut(&mut self, idx: usize) -> impl Iterator<Item = &'_ mut Cell> {
        Self::assert_house_index(idx);
        self.0.iter_mut().skip(idx * HOUSE_SIZE).take(HOUSE_SIZE)
    }

    pub fn rows(&self) -> Chunks<'_, Cell> {
        self.0.chunks(HOUSE_SIZE)
    }

    pub fn col(&self, idx: usize) -> impl Iterator<Item = &'_ Cell> {
        Self::assert_house_index(idx);
        self.0
            .iter()
            .enumerate()
            .filter_map(move |(cell_idx, cell)| {
                if Coord::from_index(cell_idx, HOUSE_SIZE).col() == idx {
                    Some(cell)
                } else {
                    None
                }
            })
    }

    pub fn col_mut(&mut self, idx: usize) -> impl Iterator<Item = &'_ mut Cell> {
        Self::assert_house_index(idx);
        self.0
            .iter_mut()
            .enumerate()
            .filter_map(move |(cell_idx, cell)| {
                if Coord::from_index(cell_idx, HOUSE_SIZE).col() == idx {
                    Some(cell)
                } else {
                    None
                }
            })
    }

    #[must_use]
    pub fn cols(&self) -> Vec<impl Iterator<Item = &'_ Cell>> {
        DIGIT_INDICES
            .collect::<Vec<usize>>()
            .iter()
            .map(|&i| self.col(i))
            .collect()
    }

    pub fn square_by_index(&self, idx: usize) -> impl Iterator<Item = &'_ Cell> {
        Self::assert_house_index(idx);
        self.square(Coord::from_index(idx, SQUARE_SIZE))
    }

    pub fn square(&self, coord: Coord) -> impl Iterator<Item = &'_ Cell> {
        let square_indices = Self::square_indices(coord);

        self.0.iter().enumerate().filter_map(move |(index, cell)| {
            if square_indices.contains(&index) {
                Some(cell)
            } else {
                None
            }
        })
    }

    pub fn square_mut_by_index(&mut self, idx: usize) -> impl Iterator<Item = &'_ mut Cell> {
        Self::assert_house_index(idx);
        self.square_mut(Coord::from_index(idx, SQUARE_SIZE))
    }

    pub fn square_mut(&mut self, coord: Coord) -> impl Iterator<Item = &'_ mut Cell> {
        let square_indices = Self::square_indices(coord);

        self.0
            .iter_mut()
            .enumerate()
            .filter_map(move |(index, cell)| {
                if square_indices.contains(&index) {
                    Some(cell)
                } else {
                    None
                }
            })
    }

    const fn square_indices(Coord(row, col): Coord) -> [usize; HOUSE_SIZE] {
        let square_row = row * HOUSE_SIZE * SQUARE_SIZE;
        let square_col = col * SQUARE_SIZE;

        let square_index = square_row + square_col;

        [
            square_index,
            square_index + 1,
            square_index + 2,
            //
            square_index + HOUSE_SIZE,
            square_index + HOUSE_SIZE + 1,
            square_index + HOUSE_SIZE + 2,
            //
            square_index + HOUSE_SIZE * 2,
            square_index + HOUSE_SIZE * 2 + 1,
            square_index + HOUSE_SIZE * 2 + 2,
        ]
    }

    #[must_use]
    pub fn is_valid(&self) -> bool {
        for row in self.rows() {
            if !row.iter().all_unique() {
                return false;
            }
        }

        for mut col in self.cols() {
            if !col.all_unique() {
                return false;
            }
        }

        for mut square in DIGIT_INDICES.map(|i| self.square_by_index(i)) {
            if !square.all_unique() {
                return false;
            }
        }

        true
    }

    #[inline]
    fn assert_house_index(idx: usize) {
        assert!(
            idx < HOUSE_SIZE,
            "house index must be between 0 and {}, got {} instead",
            HOUSE_SIZE - 1,
            idx
        );
    }
}

impl Default for Sudoku {
    fn default() -> Self {
        Self([Cell::default(); GRID_SIZE])
    }
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const PIPE: char = '|';

        f.write_str(".-----.-----.-----.\n")?;

        for (row_idx, row) in self.rows().enumerate() {
            f.write_char(PIPE)?;

            for (cell_idx, cell) in row.iter().enumerate() {
                match &cell.digit {
                    Some(digit) => write!(f, "{}", *digit)?,
                    None => f.write_char('.')?,
                }

                if (cell_idx + 1) % SQUARE_SIZE == 0 {
                    f.write_char(PIPE)?;
                } else {
                    f.write_char(' ')?;
                }
            }

            f.write_char('\n')?;

            if row_idx < HOUSE_SIZE - 1 && (row_idx + 1) % SQUARE_SIZE == 0 {
                f.write_str(":----- ----- -----:\n")?;
            }
        }

        f.write_str("'-----'-----'-----'")?;

        Ok(())
    }
}

// #[derive(Debug,Default,PartialEq, Eq)]
// pub struct Candidates(BitSet<u8>);

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Cell {
    pub(crate) digit: Option<Digit>,
    //pub(crate) candidates: Candidates,
}

#[derive(Debug, Display, Default, Deref, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Digit(pub(crate) u8);

impl Digit {
    #[inline]
    #[must_use]
    pub fn is_valid(value: u8) -> bool {
        DIGITS.contains(&value)
    }

    /// # Errors
    ///
    /// Will return `Error::InvalidDigit` if the value is not between 1-9
    #[inline]
    pub fn new(value: u8) -> SudokuResult<Self> {
        Self::try_from(value)
    }

    #[inline]
    #[must_use]
    pub const fn new_unchecked(value: u8) -> Self {
        Self(value)
    }
}

impl TryFrom<u8> for Digit {
    type Error = SudokuError;

    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if Self::is_valid(value) {
            Ok(Self(value))
        } else {
            Err(SudokuError::InvalidDigit(value))
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Coord(pub usize, pub usize);

impl Coord {
    #[inline]
    #[must_use]
    pub const fn from_index(index: usize, size: usize) -> Self {
        Self(index / size, index % size)
    }

    #[inline]
    #[must_use]
    pub const fn as_index(&self) -> usize {
        (self.0 * HOUSE_SIZE) + self.1
    }

    #[inline]
    #[must_use]
    pub const fn row(&self) -> usize {
        self.0
    }

    #[inline]
    #[must_use]
    pub const fn col(&self) -> usize {
        self.1
    }
}

pub type SudokuResult<T> = Result<T, SudokuError>;

#[derive(Error, Debug)]
pub enum SudokuError {
    #[error("digit must be between 1 and 9, got {0}")]
    InvalidDigit(u8),
}
