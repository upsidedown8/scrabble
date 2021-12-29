use crate::game::tile::Letter;

use super::{bitboard::BitBoard, play::Play, tile::Tile};
use std::{fmt, mem};

#[derive(Clone, Copy, Debug)]
pub struct Board {
    grid: [Option<Tile>; 255],
    occupancy: BitBoard,
}
impl Board {
    pub fn get_tile<T>(&self, pos: T) -> Option<Tile>
    where
        T: Into<Pos>,
    {
        self.grid[usize::from(pos.into())]
    }
    pub fn set_tile<T>(&mut self, pos: T, tile: Option<Tile>) -> Option<Tile>
    where
        T: Into<Pos>,
    {
        mem::replace(&mut self.grid[usize::from(pos.into())], tile)
    }
    pub fn make_play(&mut self, play: Play) {
        for (pos, tile) in play.tiles() {
            self.set_tile(*pos, Some(*tile));
        }
    }
}
impl Default for Board {
    fn default() -> Self {
        Self {
            grid: [None; 255],
            occupancy: BitBoard::default(),
        }
    }
}
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // print row headers
        write!(f, "   ")?;
        for col in 0..15 {
            write!(f, " {} ", Letter::from(col))?;
        }
        writeln!(f)?;

        for row in 0..15 {
            // print row header
            write!(f, "{:>2} ", row)?;

            for col in 0..15 {
                match self.get_tile((row, col)) {
                    Some(tile) => write!(f, "{}", tile)?,
                    None => write!(f, " . ")?,
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos(usize);
impl From<(usize, usize)> for Pos {
    fn from((x, y): (usize, usize)) -> Self {
        let row = x % 15;
        let col = y % 15;

        Self(row * 15 + col)
    }
}
impl From<usize> for Pos {
    fn from(pos: usize) -> Self {
        Self(pos % 225)
    }
}
impl From<Pos> for usize {
    fn from(p: Pos) -> Self {
        p.0
    }
}
impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", Letter::from(self.col() + 65), self.row())
    }
}
impl Pos {
    pub fn row(&self) -> usize {
        self.0 / 15
    }
    pub fn col(&self) -> usize {
        self.0 % 15
    }
    pub fn cartesian(&self) -> (usize, usize) {
        (self.row(), self.col())
    }
    pub fn offset(&self, dir: Direction, count: usize) -> Option<Self> {
        let vector = dir.vector(count as i32);

        // current coordinates
        let (row, col) = self.cartesian();

        // calculate new coordinates
        let row = (row as i32) + vector.0;
        let col = (col as i32) + vector.1;

        // if the new row,col are on the grid, return the `Pos`
        if (0..15).contains(&col) && (0..15).contains(&row) {
            Some(Self((row * 15 + col) as usize))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    pub fn unit_vector(&self) -> (i32, i32) {
        self.vector(1)
    }
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
