use std::{
    fmt::{Display, Write},
    hash::Hash,
    slice::Chunks,
};

use derive_more::{Deref, Display};
use itertools::Itertools;

use crate::{
    consts::{GRID_SIZE, HOUSE_SIZE, SQUARE_SIZE},
    error::SudokuError,
    generate,
};

pub type House<'a> = [&'a Cell; HOUSE_SIZE];
pub type HouseMut<'a> = [&'a mut Cell; HOUSE_SIZE];

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

impl Sudoku {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_filled() -> Self {
        generate::latin_squares()
    }

    pub fn cell(&self, coord: Coord) -> Option<&Cell> {
        self.0.get(coord.as_index())
    }

    pub fn cell_mut(&mut self, coord: Coord) -> Option<&mut Cell> {
        self.0.get_mut(coord.as_index())
    }

    pub fn row(&self, idx: usize) -> impl Iterator<Item = &'_ Cell> {
        Self::validate_house_index(idx);
        self.0.iter().skip(idx * HOUSE_SIZE).take(HOUSE_SIZE)
    }

    pub fn row_mut(&mut self, idx: usize) -> impl Iterator<Item = &'_ mut Cell> {
        Self::validate_house_index(idx);
        self.0.iter_mut().skip(idx * HOUSE_SIZE).take(HOUSE_SIZE)
    }

    pub fn rows(&self) -> Chunks<'_, Cell> {
        self.0.chunks(HOUSE_SIZE)
    }

    pub fn col(&self, idx: usize) -> impl Iterator<Item = &'_ Cell> {
        Self::validate_house_index(idx);
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
        Self::validate_house_index(idx);
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

    pub fn square_by_index(&self, idx: usize) -> impl Iterator<Item = &'_ Cell> {
        Self::validate_house_index(idx);
        let Coord(row, col) = Coord::from_index(idx, SQUARE_SIZE);
        self.square(row, col)
    }

    pub fn square(&self, row: usize, col: usize) -> impl Iterator<Item = &'_ Cell> {
        let square_indices = Self::square_indices(row, col);

        self.0.iter().enumerate().filter_map(move |(index, cell)| {
            if square_indices.contains(&index) {
                Some(cell)
            } else {
                None
            }
        })
    }

    pub fn square_mut_by_index(&mut self, idx: usize) -> impl Iterator<Item = &'_ mut Cell> {
        Self::validate_house_index(idx);
        let Coord(row, col) = Coord::from_index(idx, SQUARE_SIZE);
        self.square_mut(row, col)
    }

    pub fn square_mut(&mut self, row: usize, col: usize) -> impl Iterator<Item = &'_ mut Cell> {
        let square_indices = Self::square_indices(row, col);

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

    fn square_indices(row: usize, col: usize) -> [usize; HOUSE_SIZE] {
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

    pub fn is_valid(&self) -> bool {
        for row in self.rows() {
            if !row.iter().all_unique() {
                return false;
            }
        }

        self.col(4).for_each(|x| {
            if let Some(d) = &x.digit {
                println!("{d}");
            }
        });
        // for col in self.cols() {
        //     if !col.iter().all_unique() {
        //         return false;
        //     }
        // }

        for mut square in (0..HOUSE_SIZE).map(|i| self.square_by_index(i)) {
            if !square.all_unique() {
                return false;
            }
        }

        true
    }

    #[inline]
    fn validate_house_index(idx: usize) {
        if idx >= HOUSE_SIZE {
            panic!(
                "house index must be between 0 and {}, got {idx} instead",
                HOUSE_SIZE - 1
            );
        }
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

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Coord(pub usize, pub usize);

impl Coord {
    #[inline]
    pub fn from_index(index: usize, size: usize) -> Self {
        Self(index / size, index % size)
    }

    #[inline]
    pub fn as_index(&self) -> usize {
        (self.0 * HOUSE_SIZE) + self.1
    }

    #[inline]
    pub fn row(&self) -> usize {
        self.0
    }

    #[inline]
    pub fn col(&self) -> usize {
        self.1
    }
}
