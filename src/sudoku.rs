use std::{
    collections::HashSet,
    fmt::{Display, Write},
    hash::Hash,
    slice::Chunks,
};

use bitflags::bitflags;
use derive_more::{Deref, Display};
use rand::Rng;
use thiserror::Error;

use crate::prelude::{
    BruteForceSolver, Generate, LatinSquares, Solve, SudokuConfig, DIGITS, DIGIT_INDICES,
    GRID_SIZE, HOUSE_SIZE, SQUARE_SIZE,
};

// TODO: tests
// TODO: docs

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Sudoku(pub [Cell; GRID_SIZE]);

impl Sudoku {
    #[must_use]
    pub fn new_empty() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn new_filled(config: SudokuConfig) -> Self {
        Self::new_filled_with_generator(LatinSquares, config)
    }

    #[must_use]
    pub fn new_unique(config: SudokuConfig) -> Self {
        Self::new_with_generator(LatinSquares, config)
    }

    #[inline]
    #[must_use]
    pub const fn size(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn new_with_generator(generator: impl Generate, config: SudokuConfig) -> Self {
        generator.generate(config)
    }

    #[must_use]
    pub fn new_filled_with_generator(generator: impl Generate, config: SudokuConfig) -> Self {
        generator.generate_filled(config)
    }

    #[must_use]
    pub const fn into_inner(self) -> [Cell; GRID_SIZE] {
        self.0
    }

    #[must_use]
    pub const fn as_slice(&self) -> &[Cell] {
        &self.0
    }

    #[must_use]
    pub fn as_slice_mut(&mut self) -> &mut [Cell] {
        &mut self.0
    }

    pub fn solve_with(&mut self, solver: impl Solve) -> bool {
        solver.solve(self)
    }

    pub fn count_solutions(&self, limit: usize) -> usize {
        if self.is_filled() {
            return 1;
        }

        let mut solutions = Vec::with_capacity(limit);

        for (i, cell) in self.cells().enumerate().filter(|(_, c)| c.digit.is_none()) {
            for digit in self.cell_candidates(i).digits() {
                let mut sudoku = self.clone();

                sudoku.set_cell(cell.coord, digit);

                let solved = sudoku.solve_with(BruteForceSolver::new());

                if solved && (solutions.is_empty() || solutions.iter().any(|s| s != &sudoku)) {
                    if solutions.len() == limit {
                        return solutions.len();
                    } else {
                        solutions.push(sudoku);
                    }
                }
            }
        }

        solutions.len()
    }

    pub fn is_unique(&self) -> bool {
        self.count_solutions(2) == 1
    }

    #[must_use]
    pub fn count_filled_cells(&self) -> usize {
        self.0.iter().filter_map(|cell| cell.digit).count()
    }

    #[must_use]
    pub fn count_unfilled_cells(&self) -> usize {
        self.0.len() - self.count_filled_cells()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.count_filled_cells() == 0
    }

    #[must_use]
    pub fn is_filled(&self) -> bool {
        self.count_filled_cells() == self.0.len()
    }

    #[must_use]
    pub fn cell<I: SudokuIndex>(&self, i: I) -> Option<&Cell> {
        self.0.get(i.into_index())
    }

    pub fn cell_mut<I: SudokuIndex>(&mut self, i: I) -> Option<&mut Cell> {
        self.0.get_mut(i.into_index())
    }

    pub fn cells(&self) -> impl Iterator<Item = &'_ Cell> {
        self.0.iter()
    }

    pub fn set_cell<I: SudokuIndex>(&mut self, i: I, digit: Digit) -> Option<&mut Cell> {
        let mut cell = self.cell_mut(i.into_index())?;
        cell.digit = Some(digit);
        Some(cell)
    }

    pub fn with_cell<I: SudokuIndex>(mut self, i: I, digit: Digit) -> Self {
        self.set_cell(i, digit);
        self
    }

    pub fn clear_cell<I: SudokuIndex>(&mut self, i: I) -> Option<&mut Cell> {
        let mut cell = self.cell_mut(i.into_index())?;
        cell.digit = None;
        Some(cell)
    }

    #[inline]
    pub fn cells_mut(&mut self) -> impl Iterator<Item = &'_ mut Cell> {
        self.0.iter_mut()
    }

    pub fn row(&self, index: usize) -> impl Iterator<Item = &'_ Cell> {
        Self::assert_house_index(index);
        self.0.iter().skip(index * HOUSE_SIZE).take(HOUSE_SIZE)
    }

    pub fn row_mut(&mut self, index: usize) -> impl Iterator<Item = &'_ mut Cell> {
        Self::assert_house_index(index);
        self.0.iter_mut().skip(index * HOUSE_SIZE).take(HOUSE_SIZE)
    }

    pub fn rows(&self) -> Chunks<'_, Cell> {
        self.0.chunks(HOUSE_SIZE)
    }

    pub fn col(&self, index: usize) -> impl Iterator<Item = &'_ Cell> {
        Self::assert_house_index(index);

        self.0
            .iter()
            .enumerate()
            .filter_map(move |(cell_index, cell)| {
                if Coord::from_index(cell_index).col() == index {
                    Some(cell)
                } else {
                    None
                }
            })
    }

    pub fn col_mut(&mut self, index: usize) -> impl Iterator<Item = &'_ mut Cell> {
        Self::assert_house_index(index);

        self.0
            .iter_mut()
            .enumerate()
            .filter_map(move |(cell_index, cell)| {
                if Coord::from_index(cell_index).col() == index {
                    Some(cell)
                } else {
                    None
                }
            })
    }

    #[must_use]
    pub fn cols(&self) -> Vec<impl Iterator<Item = &'_ Cell>> {
        DIGIT_INDICES.map(|i| self.col(i)).collect()
    }

    pub fn square<I: SudokuIndex>(&self, i: I) -> impl Iterator<Item = &'_ Cell> {
        let index = i.into_index_of(SQUARE_SIZE);
        Self::assert_house_index(index);

        let square_indices = Self::square_indices(Coord::from_index_of(index, SQUARE_SIZE));

        self.0.iter().enumerate().filter_map(move |(index, cell)| {
            if square_indices.contains(&index) {
                Some(cell)
            } else {
                None
            }
        })
    }

    pub fn square_mut<I: SudokuIndex>(&mut self, i: I) -> impl Iterator<Item = &'_ mut Cell> {
        let index = i.into_index_of(SQUARE_SIZE);
        Self::assert_house_index(index);

        let square_indices = Self::square_indices(Coord::from_index_of(index, SQUARE_SIZE));

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

    pub fn square_of_cell<I: SudokuIndex>(&self, i: I) -> impl Iterator<Item = &'_ Cell> {
        let coord = Coord::from_index(i.into_index());
        self.square(Self::cell_coord_to_square_coord(coord))
    }

    pub fn square_mut_of_cell<I: SudokuIndex>(
        &mut self,
        i: I,
    ) -> impl Iterator<Item = &'_ mut Cell> {
        let coord = Coord::from_index(i.into_index());
        self.square_mut(Self::cell_coord_to_square_coord(coord))
    }

    const fn cell_coord_to_square_coord(cell_coord: Coord) -> Coord {
        Coord(
            cell_coord.row() / SQUARE_SIZE,
            cell_coord.col() / SQUARE_SIZE,
        )
    }

    pub const fn square_indices(Coord(row, col): Coord) -> [usize; HOUSE_SIZE] {
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

    pub const fn square_indices_of_cell(Coord(row, col): Coord) -> [usize; HOUSE_SIZE] {
        Self::square_indices(Coord(row / SQUARE_SIZE, col / SQUARE_SIZE))
    }

    pub fn cell_candidates<I: SudokuIndex>(&self, i: I) -> Candidates {
        let index = i.into_index();
        let mut candidates = Candidates::all();

        let Coord(row, col) = Coord::from_index(index);

        for cell in self
            .row(row)
            .chain(self.col(col))
            .chain(self.square_of_cell(index))
        {
            if let Some(digit) = cell.digit {
                candidates.remove(digit);
            }
        }

        candidates
    }

    pub fn all_candidates(&self) -> Vec<Candidates> {
        self.cells()
            .enumerate()
            .map(|(i, cell)| {
                if cell.digit.is_some() {
                    Candidates::empty()
                } else {
                    self.cell_candidates(i)
                }
            })
            .collect::<Vec<_>>()
    }

    pub fn solve_all_candidates(&mut self) {
        let all_candidates = self.all_candidates();

        self.cells_mut()
            .enumerate()
            .for_each(|(i, cell)| cell.candidates = all_candidates[i]);
    }

    #[must_use]
    pub fn is_valid(&self) -> bool {
        fn house_is_unique<'a>(house_iter: impl Iterator<Item = &'a Cell>) -> bool {
            let mut used = HashSet::new();
            house_iter
                .filter(|cell| cell.digit.is_some())
                .all(move |cell| used.insert(cell))
        }

        self.rows().all(|row| house_is_unique(row.iter()))
            && self.cols().into_iter().all(house_is_unique)
            && DIGIT_INDICES.map(|i| self.square(i)).all(house_is_unique)
    }

    #[inline]
    #[track_caller]
    fn assert_house_index(index: usize) {
        assert!(
            index < HOUSE_SIZE,
            "house index must be between 0 and {}, got {} instead",
            HOUSE_SIZE - 1,
            index
        );
    }

    pub fn to_str_line(&self) -> String {
        self.cells()
            .map(|cell| {
                cell.digit
                    .map_or('0', |d| char::from_digit((*d).into(), 10).unwrap())
            })
            .collect::<String>()
    }

    pub fn from_str_line(s: &str) -> Self {
        Self::try_from(
            s.chars()
                .enumerate()
                .map(|(i, c)| Cell {
                    coord: Coord::from_index(i),
                    digit: if c == '0' {
                        None
                    } else {
                        Some(Digit::new(c.to_digit(10).unwrap().try_into().unwrap()).unwrap())
                    },
                    candidates: Candidates::empty(),
                    is_given: c != '0',
                })
                .collect::<Vec<_>>(),
        )
        .unwrap()
    }
}

impl From<[Cell; GRID_SIZE]> for Sudoku {
    fn from(value: [Cell; GRID_SIZE]) -> Self {
        Self(value)
    }
}

impl TryFrom<Vec<Cell>> for Sudoku {
    type Error = Vec<Cell>;

    fn try_from(value: Vec<Cell>) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into()?))
    }
}

impl Default for Sudoku {
    fn default() -> Self {
        let mut cells = [Cell::default(); GRID_SIZE];

        for (index, cell) in cells.iter_mut().enumerate() {
            cell.coord = Coord::from_index(index);
        }

        Self(cells)
    }
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const PIPE: char = '|';

        f.write_str(".-----.-----.-----.\n")?;

        for (row_index, row) in self.rows().enumerate() {
            f.write_char(PIPE)?;

            for (cell_index, cell) in row.iter().enumerate() {
                match &cell.digit {
                    Some(digit) => write!(f, "{}", *digit)?,
                    None => f.write_char('.')?,
                }

                if (cell_index + 1) % SQUARE_SIZE == 0 {
                    f.write_char(PIPE)?;
                } else {
                    f.write_char(' ')?;
                }
            }

            f.write_char('\n')?;

            if row_index < HOUSE_SIZE - 1 && (row_index + 1) % SQUARE_SIZE == 0 {
                f.write_str(":----- ----- -----:\n")?;
            }
        }

        f.write_str("'-----'-----'-----'")?;

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("invalid character `{char}` at index {index}")]
    InvalidChar { char: char, index: usize },
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Cell {
    pub coord: Coord,
    pub digit: Option<Digit>,
    pub candidates: Candidates,
    pub is_given: bool,
}

#[derive(Debug, Display, Default, Deref, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Digit(pub u8);

impl Digit {
    #[must_use]
    #[inline]
    pub fn is_valid(value: u8) -> bool {
        DIGITS.contains(&value)
    }

    pub fn new(value: u8) -> SudokuResult<Self> {
        Self::try_from(value)
    }

    #[must_use]
    #[inline]
    pub const fn new_unchecked(value: u8) -> Self {
        Self(value)
    }
}

impl TryFrom<u8> for Digit {
    type Error = SudokuError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if Self::is_valid(value) {
            Ok(Self(value))
        } else {
            Err(SudokuError::InvalidDigit(value))
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Candidates(CandidatesInner);

impl Candidates {
    #[must_use]
    #[inline]
    pub const fn empty() -> Self {
        Self(CandidatesInner::empty())
    }

    #[must_use]
    #[inline]
    pub const fn all() -> Self {
        Self(CandidatesInner::ALL)
    }

    pub fn add(&mut self, digit: Digit) {
        self.0.insert(digit.into());
    }

    pub fn remove(&mut self, digit: Digit) {
        self.0.remove(digit.into());
    }

    pub fn toggle(&mut self, digit: Digit) {
        self.0.toggle(digit.into());
    }

    #[must_use]
    pub fn contains(&self, digit: Digit) -> bool {
        self.0.contains(digit.into())
    }

    // PERF: inefficient
    pub fn digits(&self) -> impl Iterator<Item = Digit> + '_ {
        DIGITS
            .map(Digit::new_unchecked)
            .filter(|digit| self.contains(*digit))
    }

    // PERF: inefficient
    pub fn count(&self) -> usize {
        self.digits().count()
    }
}

impl Default for Candidates {
    fn default() -> Self {
        Self(CandidatesInner::empty())
    }
}

impl From<Digit> for Candidates {
    fn from(value: Digit) -> Self {
        let mut candidates = Self::empty();
        candidates.add(value);
        candidates
    }
}

bitflags! {
    struct CandidatesInner: usize {
        const _1 = 0b0000_0000_0000_0001;
        const _2 = 0b0000_0000_0000_0010;
        const _3 = 0b0000_0000_0000_0100;
        const _4 = 0b0000_0000_0000_1000;
        const _5 = 0b0000_0000_0001_0000;
        const _6 = 0b0000_0000_0010_0000;
        const _7 = 0b0000_0000_0100_0000;
        const _8 = 0b0000_0000_1000_0000;
        const _9 = 0b0000_0001_0000_0000;
        const ALL = Self::_1.bits | Self::_2.bits | Self::_3.bits | Self::_4.bits
                    | Self::_5.bits | Self::_6.bits | Self::_7.bits | Self::_8.bits | Self::_9.bits;
    }
}

impl From<Digit> for CandidatesInner {
    fn from(value: Digit) -> Self {
        match value.0 {
            1 => Self::_1,
            2 => Self::_2,
            3 => Self::_3,
            4 => Self::_4,
            5 => Self::_5,
            6 => Self::_6,
            7 => Self::_7,
            8 => Self::_8,
            9 => Self::_9,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Coord(pub usize, pub usize);

impl Coord {
    #[inline]
    #[must_use]
    pub fn random(rng: &mut impl Rng) -> Self {
        let random_row = rng.gen_range(DIGIT_INDICES);
        let random_col = rng.gen_range(DIGIT_INDICES);
        Self(random_row, random_col)
    }

    #[inline]
    #[must_use]
    pub const fn from_index(index: usize) -> Self {
        Self::from_index_of(index, HOUSE_SIZE)
    }

    #[inline]
    #[must_use]
    pub const fn from_index_of(index: usize, size: usize) -> Self {
        Self(index / size, index % size)
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

    #[inline]
    #[must_use]
    pub const fn with_row(self, row: usize) -> Self {
        Self(row, self.col())
    }

    #[inline]
    #[must_use]
    pub const fn with_col(self, col: usize) -> Self {
        Self(self.row(), col)
    }
}

pub trait SudokuIndex
where
    Self: Sized,
{
    fn into_index_of(self, size: usize) -> usize;

    fn into_index(self) -> usize {
        self.into_index_of(HOUSE_SIZE)
    }
}

impl SudokuIndex for usize {
    #[inline]
    fn into_index_of(self, _: usize) -> usize {
        self
    }
}

impl SudokuIndex for Coord {
    #[inline]
    #[must_use]
    fn into_index_of(self, size: usize) -> usize {
        (self.row() * size) + self.col()
    }
}

pub type SudokuResult<T> = Result<T, SudokuError>;

#[derive(Error, Debug)]
pub enum SudokuError {
    #[error("digit must be between 1 and 9, got {0}")]
    InvalidDigit(u8),
}
