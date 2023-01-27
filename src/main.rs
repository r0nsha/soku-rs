use std::ops::Deref;
use thiserror::Error;

fn main() -> Result<(), SudokuError> {
    println!("Hello, world!");
    let d = Digit::try_from(10)?;
    Ok(())
}

pub const HOUSE_SIZE: u8 = 9;

// TODO: Grid
// TODO: Given
// TODO: House
// TODO: Box
// TODO: Row
// TODO: Column
// TODO: Cell
// TODO: Candidate
// TODO: Digit

#[derive(Error, Debug)]
pub enum SudokuError {
    #[error("digit must be between 1 and 9, got {0}")]
    InvalidDigit(u8),
}

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
