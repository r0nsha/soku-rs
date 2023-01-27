use std::{
    fmt::{Display, Write},
    ops::Deref,
};

use crate::{consts::HOUSE_SIZE, error::SudokuError};

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Sudoku {
    grid: Grid,
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const PIPE: char = '|';

        f.write_str(".-----.-----.-----.\n")?;

        for (row_index, row) in self.grid.rows.iter().enumerate() {
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

            if row_index < self.grid.rows.len() - 1 && (row_index + 1) % 3 == 0 {
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
        todo!()
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Grid {
    rows: [[Cell; HOUSE_SIZE]; HOUSE_SIZE],
}

// #[derive(Debug,Default,PartialEq, Eq)]
// pub struct Candidates(BitSet<u8>);

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Cell {
    digit: Option<Digit>,
    // candidates: Candidates,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Digit(u8);

impl TryFrom<u8> for Digit {
    type Error = SudokuError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if (1..=9).contains(&value) {
            Ok(Self(value))
        } else {
            Err(SudokuError::InvalidDigit(value))
        }
    }
}

impl Deref for Digit {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
