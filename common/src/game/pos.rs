//! Module containing newtypes representing checked board [`position`](Pos)s,
//! [`row`](Row)s, [`column`](Col)s and orthagonal directions.

use crate::game::{
    board::{CELLS, COLS, ROWS},
    tile::Letter,
};
use std::fmt;

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

/// The four orthagonal directions from a point
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
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
