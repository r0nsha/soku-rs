use std::ops::Deref;

use crate::error::SudokuError;

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
