use std::ops::RangeInclusive;

pub const GRID_SIZE: usize = HOUSE_SIZE * HOUSE_SIZE;
pub const HOUSE_SIZE: usize = SQUARE_SIZE * SQUARE_SIZE;
pub const SQUARE_SIZE: usize = 3;
pub const DIGITS: RangeInclusive<u8> = 1..=9;
