use std::{
    fmt::{Display, Write},
    slice::Chunks,
};

use derive_more::Deref;

use crate::{
    consts::{GRID_SIZE, HOUSE_SIZE, SQUARE_SIZE},
    error::SudokuError,
    generate,
};

pub type House<'a> = [&'a Cell; HOUSE_SIZE];
pub type HouseMut<'a> = [&'a mut Cell; HOUSE_SIZE];
pub type HouseCoords = [Coord; HOUSE_SIZE];

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

    pub fn cell(&self, coord: Coord) -> Option<&Cell> {
        self.0.get(coord.as_index())
    }

    pub fn cell_mut(&mut self, coord: Coord) -> Option<&mut Cell> {
        self.0.get_mut(coord.as_index())
    }

    pub fn rows(&self) -> Chunks<'_, Cell> {
        self.0.chunks(HOUSE_SIZE)
    }

    pub fn square_coords(&self, row: usize, col: usize) -> Option<HouseCoords> {
        if Self::is_valid_pos(row, col) {
            return None;
        }

        let mut square = HouseCoords::default();
        let row = row * SQUARE_SIZE;
        let col = col * SQUARE_SIZE;

        for (square_index, coord) in square.iter_mut().enumerate() {
            let row_offset = square_index / SQUARE_SIZE;
            let col_offset = square_index % SQUARE_SIZE;
            *coord = Coord(row + row_offset, col + col_offset);
        }

        Some(square)
    }

    pub fn square(&self, row: usize, col: usize) -> Option<House<'_>> {
        self.square_coords(row, col)
            .map(|coords| coords.map(|coord| self.cell(coord).unwrap()))
    }

    pub fn squares(&self) -> Squares<'_> {
        Squares {
            sudoku: self,
            row: 0,
            col: 0,
        }
    }

    fn is_valid_pos(row: usize, col: usize) -> bool {
        row > 0 && row < SQUARE_SIZE && col > 0 && col < SQUARE_SIZE
    }
}

pub struct Squares<'a> {
    sudoku: &'a Sudoku,
    row: usize,
    col: usize,
}

impl<'a> Iterator for Squares<'a> {
    type Item = House<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= HOUSE_SIZE {
            return None;
        }

        let square = self.sudoku.square(self.row, self.col).unwrap();

        if self.col == SQUARE_SIZE - 1 {
            self.row += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }

        Some(square)
    }
}

// TODO: is there a safe way to implement SquaresMut?

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

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Coord(pub usize, pub usize);

impl Coord {
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
