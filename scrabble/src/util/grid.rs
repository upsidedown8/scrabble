//! Contains an implementation of a [`Grid`] type that is reused
//! for horizontal and vertical boards.

use crate::{
    game::{board::CELLS, tile::Tile},
    util::{bitboard::BitBoard, pos::Direction, pos::Pos, words::WordBoundaries},
};
use std::ops::Index;

/// A structure to store the positions of tiles on the board, either
/// horizontally or vertically. Implements the [`Index`] trait, so elements
/// can be accessed as `grid[pos`.
#[derive(Debug, Clone, Copy)]
pub struct Grid {
    tiles: [Option<Tile>; CELLS],
    occ: BitBoard,
    dir: Direction,
}
impl Index<Pos> for Grid {
    type Output = Option<Tile>;

    fn index(&self, index: Pos) -> &Self::Output {
        &self.tiles[usize::from(index)]
    }
}
impl Grid {
    /// Constructs a new grid from a direction.
    pub fn new(dir: Direction) -> Self {
        assert!(matches!(dir, Direction::South | Direction::East));

        Self {
            tiles: [None; CELLS],
            occ: BitBoard::default(),
            dir,
        }
    }
    /// Sets the tile at `pos` on the grid.
    pub fn set(&mut self, pos: Pos, tile: Option<Tile>) {
        match tile {
            Some(_) => self.occ.set(pos),
            None => self.occ.clear(pos),
        };
        self.tiles[usize::from(pos)] = tile;
    }
    /// Gets a bitboard storing the occupancy for the grid,
    pub fn occ(&self) -> &BitBoard {
        &self.occ
    }
    /// Gets an iterator over the [`WordBoundaries`] on the grid.
    pub fn word_boundaries(&self) -> WordBoundaries {
        WordBoundaries::new(*self.occ())
    }
    /// Converts the position back to its horizontal coordinate.
    pub fn map_pos(&self, pos: Pos) -> Pos {
        match self.dir {
            Direction::East => pos,
            Direction::South => pos.clockwise90(),
            _ => unreachable!(),
        }
    }
}
