//! Models the scrabble board.

use crate::game::{
    bitboard::BitBoard,
    play::Play,
    pos::{Col, Pos, Row},
    tile::Tile,
};
use std::{fmt, mem};

/// The number of rows on the board.
pub const ROWS: usize = 15;
/// The number of columns on the board.
pub const COLS: usize = 15;
/// The number of squares on the board.
pub const CELLS: usize = 15 * 15;

/// Represents the 15 x 15 scrabble board, storing the location of
/// tiles, and allowing [`Play`]s to be made and validated.
#[derive(Clone, Debug)]
pub struct Board {
    grid: [Option<Tile>; CELLS],
    occupancy: BitBoard,
}
impl Board {
    /// Gets the tile at `pos`
    pub fn get_tile<T>(&self, pos: T) -> Option<Tile>
    where
        T: Into<Pos>,
    {
        self.grid[usize::from(pos.into())]
    }
    /// Sets the tile at `pos`
    pub fn set_tile<T>(&mut self, pos: T, tile: Option<Tile>) -> Option<Tile>
    where
        T: Into<Pos>,
    {
        mem::replace(&mut self.grid[usize::from(pos.into())], tile)
    }
    /// Attempts to perform a [`Play`] on the board
    pub fn make_play(&mut self, play: Play) {
        for (pos, tile) in play.tiles() {
            self.set_tile(*pos, Some(*tile));
        }
    }
}
impl Default for Board {
    fn default() -> Self {
        Self {
            grid: [None; CELLS],
            occupancy: BitBoard::default(),
        }
    }
}
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // print col headers
        write!(f, "   ")?;
        for col in Col::iter() {
            write!(f, " {} ", col)?;
        }
        writeln!(f)?;

        for row in Row::iter() {
            // print row header
            write!(f, "{:>2} ", row.to_string())?;

            for col in Col::iter() {
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
