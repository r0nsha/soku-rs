use std::ops::{Range, RangeInclusive};

pub const GRID_SIZE: usize = HOUSE_SIZE * HOUSE_SIZE;
pub const HOUSE_SIZE: usize = SQUARE_SIZE * SQUARE_SIZE;
pub const SQUARE_SIZE: usize = 3;

#[allow(clippy::cast_possible_truncation)]
pub const DIGITS: RangeInclusive<u8> = 1..=HOUSE_SIZE as u8;
pub const DIGIT_INDICES: Range<usize> = 0..HOUSE_SIZE;
