use crate::{cell::Cell, consts::HOUSE_SIZE};

#[derive(Debug)]
pub struct Grid {
    inner: [[Cell; HOUSE_SIZE]; HOUSE_SIZE],
}
