use crate::{candidates::Candidates, digit::Digit};

#[derive(Debug)]
pub struct Cell {
    digit: Option<Digit>,
    candidates: Candidates,
}
