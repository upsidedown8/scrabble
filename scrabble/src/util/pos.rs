//! Module containing newtypes representing checked board [`Pos`]itions,
//! [`Row`]s, [`Col`]umns and orthagonal directions.

use crate::game::{
    board::{CELLS, COLS, ROWS},
    tile::Letter,
};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Additional bonus for certain positions on the board.
#[derive(Debug, Clone, Copy, PartialEq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Serialize, Deserialize)]
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
impl fmt::Debug for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pos({})", self)
    }
}
impl Pos {
    /// Swaps the row and column of a [`Pos`].
    pub fn swap_rc(&self) -> Pos {
        let row = Row::from(usize::from(self.col()));
        let col = Col::from(usize::from(self.row()));

        Pos::from((row, col))
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
    /// Finds the pos in the grid offset by 1 in the given direction.
    pub fn dir(&self, dir: Direction) -> Option<Self> {
        self.offset(dir, 1)
    }
    /// Finds the pos in the grid, offset by `count` in direction `dir`
    pub fn offset(&self, dir: Direction, count: usize) -> Option<Self> {
        let (drow, dcol) = dir.vector(count);

        // current coordinates
        let (row, col) = self.row_col();

        // calculate new coordinates
        let row = usize::from(row) as i32 + drow;
        let col = usize::from(col) as i32 + dcol;

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
    /// Gets an iterator containing all positions from the current one
    /// in the given direction.
    pub fn project(self, dir: Direction) -> impl Iterator<Item = Pos> {
        std::iter::successors(Some(self), move |pos| pos.dir(dir))
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
impl From<Row> for usize {
    #[inline]
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
    /// The first row.
    pub fn first() -> Self {
        Row(0)
    }
    /// The last row.
    pub fn last() -> Self {
        Row(ROWS - 1)
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
impl From<Col> for usize {
    #[inline]
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
        Col(0)
    }
    /// The last column
    pub fn last() -> Self {
        Col(COLS - 1)
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
    /// Gets a `scale`d vector in the `Direction` represented by `self`.
    pub fn vector(&self, scale: usize) -> (i32, i32) {
        let scale = scale as i32;
        match self {
            Direction::North => (-scale, 0),
            Direction::South => (scale, 0),
            Direction::West => (0, -scale),
            Direction::East => (0, scale),
        }
    }
    /// Gets the opposite direction.
    pub fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
    /// Gets the perpendicular direction.
    pub fn perpendicular(&self) -> Self {
        match self {
            Direction::East => Direction::South,
            Direction::South => Direction::East,
            Direction::North => Direction::West,
            Direction::West => Direction::North,
        }
    }
}
