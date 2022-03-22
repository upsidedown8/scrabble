use crate::{
    game::{board::CELLS, tile::Tile},
    util::{bitboard::BitBoard, pos::Pos, words::WordBoundaries},
};
use std::ops::Index;

/// A structure to store the positions of tiles on the board, either
/// horizontally or vertically. Implements the [`Index`] trait, so elements
/// can be accessed as `grid[pos`.
#[derive(Debug, Clone, Copy)]
pub struct Grid {
    tiles: [Option<Tile>; CELLS],
    occ: BitBoard,
}
impl Default for Grid {
    fn default() -> Grid {
        Grid {
            tiles: [None; CELLS],
            occ: BitBoard::default(),
        }
    }
}
impl Index<Pos> for Grid {
    type Output = Option<Tile>;

    fn index(&self, index: Pos) -> &Self::Output {
        &self.tiles[usize::from(index)]
    }
}
impl Grid {
    /// Sets the tile at `pos` on the grid.
    pub fn set(&mut self, pos: Pos, tile: impl Into<Option<Tile>>) {
        let tile = tile.into();
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
}
