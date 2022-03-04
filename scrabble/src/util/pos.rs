//! Module containing newtypes representing checked board [`Pos`]itions,
//! [`Row`]s, [`Col`]umns and orthagonal directions.

use serde::{Deserialize, Serialize};

use crate::game::{
    board::{CELLS, COLS, ROWS},
    tile::Letter,
};
use std::{fmt, ops::Sub};

/// Additional bonus for certain positions on the board.
#[derive(Debug, Clone, Copy)]
pub enum Premium {
    /// The square doubles the value of the tile placed on it
    DoubleLetter,
    /// The square triples the value of the tile placed on it
    TripleLetter,
    /// The square doubles the total value of a word placed on it
    DoubleWord,
    /// The square triples the total value of a word placed on it
    TripleWord,
    /// The center square: counts as a double word. The first word
    /// must intersect this square.
    Start,
}

impl Premium {
    /// Gets the multiplier for a word placed on a square with
    /// this bonus.
    pub fn word_multiplier(&self) -> usize {
        match self {
            Premium::DoubleLetter => 1,
            Premium::TripleLetter => 1,
            Premium::DoubleWord => 2,
            Premium::TripleWord => 3,
            Premium::Start => 2,
        }
    }
    /// Gets the multiplier for a tile placed on a square with
    /// this bonus.
    pub fn tile_multiplier(&self) -> usize {
        match self {
            Premium::DoubleLetter => 2,
            Premium::TripleLetter => 3,
            Premium::DoubleWord => 1,
            Premium::TripleWord => 1,
            Premium::Start => 1,
        }
    }
}

/// A position on the board. Ranges from `0..`[`CELLS`].
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Serialize, Deserialize)]
pub struct Pos(usize);

impl From<usize> for Pos {
    fn from(pos: usize) -> Self {
        Self(pos % (ROWS * COLS))
    }
}
impl From<Pos> for usize {
    fn from(p: Pos) -> Self {
        p.0
    }
}
impl<R: Into<Row>, C: Into<Col>> From<(R, C)> for Pos {
    fn from((r, c): (R, C)) -> Self {
        let row = usize::from(r.into());
        let col = usize::from(c.into());

        Self(row * COLS + col)
    }
}
impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.col(), self.row())
    }
}
impl Pos {
    /// Rotates a `pos` 90 degrees anticlockwise about the center square.
    pub fn anti_clockwise90(&self) -> Pos {
        let (r, c) = self.row_col();

        let r_prime = Row::from(14 - usize::from(c));
        let c_prime = Col::from(usize::from(r));

        Pos::from((r_prime, c_prime))
    }
    /// Rotates a `pos` 90 degrees clockwise about the center square. Inverse
    /// functon of `rotate_90_anti_clockwise`.
    pub fn clockwise90(&self) -> Pos {
        let (r, c) = self.row_col();

        let r_prime = Row::from(usize::from(c));
        let c_prime = Col::from(14 - usize::from(r));

        Pos::from((r_prime, c_prime))
    }
    /// Gets the `Pos` for the start square.
    pub fn start() -> Self {
        Self::from((7, 7))
    }
    /// Checks whether the `Pos` is the start square.
    pub fn is_start(&self) -> bool {
        *self == Self::start()
    }
    /// Gets the optional tile bonus of the `Pos`.
    pub fn premium(&self) -> Option<Premium> {
        let (row, col) = self.row_col();

        // finds positive difference between two unsigned numbers
        let abs_diff = |a, b| match a > b {
            true => a - b,
            false => b - a,
        };

        // find difference to start square
        let delta_row = abs_diff(usize::from(row), 7);
        let delta_col = abs_diff(usize::from(col), 7);

        match (delta_row, delta_col) {
            (0, 0) => Some(Premium::Start),
            (2, 2) | (2, 6) | (6, 2) => Some(Premium::TripleLetter),
            (0, 4) | (4, 0) | (1, 1) | (1, 5) | (5, 1) | (7, 4) | (4, 7) => {
                Some(Premium::DoubleLetter)
            }
            (7, 7) | (0, 7) | (7, 0) => Some(Premium::TripleWord),
            (a, b) if a == b => Some(Premium::DoubleWord),
            _ => None,
        }
    }
    /// Gets the tuple (tile_multiplier, word_multiplier) for the position.
    /// Defaults to (1, 1).
    pub fn premium_multipliers(&self) -> (usize, usize) {
        match self.premium() {
            Some(bonus) => (bonus.tile_multiplier(), bonus.word_multiplier()),
            _ => (1, 1),
        }
    }
    /// Gets the row number
    pub fn row(&self) -> Row {
        Row::from((self.0 / COLS) % ROWS)
    }
    /// Gets the column number
    pub fn col(&self) -> Col {
        Col::from(self.0 % COLS)
    }
    /// Gets the pair (row, col) for the coordinate
    pub fn row_col(&self) -> (Row, Col) {
        (self.row(), self.col())
    }
    /// Finds the pos in the grid, offset by `count` in direction `dir`
    pub fn offset(&self, dir: Direction, count: usize) -> Option<Self> {
        let vector = dir.vector(count as i32);

        // current coordinates
        let (row, col) = self.row_col();

        // calculate new coordinates
        let row = i32::from(row) + vector.0;
        let col = i32::from(col) + vector.1;

        // if the new row,col are on the grid, return the `Pos`
        if (0..COLS as i32).contains(&col) && (0..ROWS as i32).contains(&row) {
            Some(Pos::from((row as usize, col as usize)))
        } else {
            None
        }
    }
    /// Returns an iterator over all board positions.
    pub fn iter() -> impl Iterator<Item = Self> {
        (0..CELLS).map(Pos::from)
    }
    /// Advances the position value by 1.
    pub fn next(&self) -> Self {
        let &Pos(val) = self;

        Pos((val + 1) % CELLS)
    }
}
impl Sub<Pos> for Pos {
    type Output = Pos;

    fn sub(self, rhs: Pos) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

/// A vertical coordinate from `0..=14`
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Row(usize);

impl From<usize> for Row {
    fn from(row: usize) -> Self {
        Row(row % ROWS)
    }
}
impl From<Row> for i32 {
    fn from(row: Row) -> Self {
        row.0 as i32
    }
}
impl From<Row> for usize {
    fn from(row: Row) -> Self {
        row.0
    }
}
impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Row {
    /// The first row
    pub fn first() -> Self {
        Self::from(0)
    }
    /// The last row
    pub fn last() -> Self {
        Self::from(ROWS - 1)
    }
    /// Returns an iterator over all columns
    pub fn iter() -> impl Iterator<Item = Self> {
        (0..ROWS).map(Row::from)
    }
}

/// A horizontal coordinate from `A..=O`
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Col(usize);

impl From<usize> for Col {
    fn from(col: usize) -> Self {
        Col(col % COLS)
    }
}
impl From<Col> for i32 {
    fn from(col: Col) -> Self {
        col.0 as i32
    }
}
impl From<Col> for usize {
    fn from(col: Col) -> Self {
        col.0
    }
}
impl fmt::Display for Col {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Letter::from(self.0))
    }
}
impl Col {
    /// The first column
    pub fn first() -> Self {
        Self::from(0)
    }
    /// The last column
    pub fn last() -> Self {
        Self::from(COLS - 1)
    }
    /// Returns an iterator over all columns
    pub fn iter() -> impl Iterator<Item = Self> {
        (0..COLS).map(Col::from)
    }
}

/// The four orthagonal directions from a [`Pos`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// Up
    North,
    /// Right
    East,
    /// Down
    South,
    /// Left
    West,
}
impl Direction {
    /// Gets a unit vector in the `Direction` represented by `self`
    pub fn unit_vector(&self) -> (i32, i32) {
        self.vector(1)
    }
    /// Gets a `scale`d vector in the `Direction` represented by `self`
    pub fn vector(&self, scale: i32) -> (i32, i32) {
        use Direction::*;
        match self {
            North => (-scale, 0),
            South => (scale, 0),
            West => (0, -scale),
            East => (0, scale),
        }
    }
}
