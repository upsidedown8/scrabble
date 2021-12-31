//! Module containing newtypes representing checked board [`position`](Pos)s,
//! [`row`](Row)s, [`column`](Col)s and orthagonal directions.

use crate::game::{
    board::{CELLS, COLS, ROWS},
    tile::Letter,
};
use std::fmt;

/// Additional bonus for certain positions on the board.
#[derive(Debug, Clone, Copy)]
pub enum PosBonus {
    /// The square doubles the value of the tile placed on it
    DoubleLetter,
    /// The square triples the value of the tile placed on it
    TripleLetter,
    /// The square doubles the total value of a word placed on it
    DoubleWord,
    /// The square triples the total value of a word placed on it
    TripleWord,
    /// The center square: counts as a double letter. The first word
    /// must intersect this square.
    Start,
}

impl PosBonus {
    /// Gets the multiplier for a word placed on a square with
    /// this bonus.
    pub fn word_multiplier(&self) -> usize {
        match self {
            PosBonus::DoubleLetter => 1,
            PosBonus::TripleLetter => 1,
            PosBonus::DoubleWord => 2,
            PosBonus::TripleWord => 3,
            PosBonus::Start => 1,
        }
    }
    /// Gets the multiplier for a tile placed on a square with
    /// this bonus.
    pub fn letter_multiplier(&self) -> usize {
        match self {
            PosBonus::DoubleLetter => 2,
            PosBonus::TripleLetter => 3,
            PosBonus::DoubleWord => 1,
            PosBonus::TripleWord => 1,
            PosBonus::Start => 2,
        }
    }
}

/// A position on the board. Ranges from `0..=`[`CELLS`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos(usize);

impl<R, C> From<(R, C)> for Pos
where
    R: Into<Row>,
    C: Into<Col>,
{
    fn from((r, c): (R, C)) -> Self {
        let row = usize::from(r.into());
        let col = usize::from(c.into());

        Self(row * COLS + col)
    }
}
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
impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.col(), self.row())
    }
}
impl Pos {
    /// Gets the `Pos` for the start square.
    pub fn start() -> Self {
        Self::from((7, 7))
    }
    /// Checks whether the `Pos` is the start square.
    pub fn is_start(&self) -> bool {
        *self == Self::start()
    }
    /// Gets the optional tile bonus of the `Pos`.
    pub fn bonus(&self) -> Option<PosBonus> {
        let (row, col) = self.cartesian();

        // finds positive difference between two unsigned numbers
        let abs_diff = |a, b| match a > b {
            true => a - b,
            false => b - a,
        };

        // find difference to start square
        let delta_row = abs_diff(usize::from(row), 7);
        let delta_col = abs_diff(usize::from(col), 7);

        match (delta_row, delta_col) {
            (0, 0) => Some(PosBonus::Start),
            (2, 2) | (2, 6) | (6, 2) => Some(PosBonus::TripleLetter),
            (0, 4) | (4, 0) | (1, 1) | (1, 5) | (5, 1) | (7, 4) | (4, 7) => {
                Some(PosBonus::DoubleLetter)
            }
            (7, 7) | (0, 7) | (7, 0) => Some(PosBonus::TripleWord),
            (a, b) if a == b => Some(PosBonus::DoubleWord),
            _ => None,
        }
    }
    /// Gets the vertical cartesian coordinate
    pub fn row(&self) -> Row {
        Row::from((self.0 / COLS) % ROWS)
    }
    /// Gets the horizontal cartesian coordinate
    pub fn col(&self) -> Col {
        Col::from(self.0 % COLS)
    }
    /// Gets the cartesian coordinates as a pair
    pub fn cartesian(&self) -> (Row, Col) {
        (self.row(), self.col())
    }
    /// Finds the pos in the grid, offset by `count` in direction `dir`
    pub fn offset(&self, dir: Direction, count: usize) -> Option<Self> {
        let vector = dir.vector(count as i32);

        // current coordinates
        let (row, col) = self.cartesian();

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
}

/// A vertical coordinate from `0..=14`
#[derive(Debug, Clone, Copy)]
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
    /// Returns an iterator over all columns
    pub fn iter() -> impl Iterator<Item = Self> {
        (0..ROWS).map(Row::from)
    }
}

/// A horizontal coordinate from `A..=O`
#[derive(Debug, Clone, Copy)]
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
    /// Returns an iterator over all columns
    pub fn iter() -> impl Iterator<Item = Self> {
        (0..COLS).map(Col::from)
    }
}

/// The four orthagonal directions from a [`Pos`].
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    /// Up
    Up,
    /// Down
    Down,
    /// Left
    Left,
    /// Right
    Right,
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
            Up => (-scale, 0),
            Down => (scale, 0),
            Left => (0, -scale),
            Right => (0, scale),
        }
    }
}
