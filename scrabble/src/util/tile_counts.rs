//! Contains a structure for keeping track of how many of each
//! of the 27 tiles are in a container.

use crate::game::tile::Tile;
use std::iter::repeat;

/// Reusable structure used to store a quantity of each tile.
#[derive(Debug, Clone, Copy)]
pub struct TileCounts {
    counts: [usize; 27],
    len: usize,
}

impl TileCounts {
    /// The number of tiles in `self`
    pub fn len(&self) -> usize {
        self.len
    }
    /// Checks whether the counts are empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// An iterator over the tiles in `self`
    pub fn iter(&self) -> impl Iterator<Item = Tile> + '_ {
        self.counts
            .iter()
            .enumerate()
            .flat_map(|(tile, &count)| repeat(Tile::from(tile)).take(count))
    }
    /// Gets the count for a specific tile
    pub fn count<T>(&self, tile: T) -> usize
    where
        T: Into<Tile>,
    {
        self.counts[usize::from(tile.into())]
    }
    /// Checks whether the `tiles` are contained within the counts.
    pub fn contains<I>(&self, tiles: I) -> bool
    where
        I: Iterator<Item = Tile>,
    {
        Self::counts(tiles)
            .into_iter()
            .zip(self.counts)
            .all(|(a, b)| a <= b)
    }
    /// Removes tiles from `self`. `self` should have sufficient tiles
    /// otherwise this method may panic.
    pub fn remove<I>(&mut self, tiles: I)
    where
        I: Iterator<Item = Tile>,
    {
        let counts = Self::counts(tiles);

        self.counts
            .iter_mut()
            .zip(counts)
            .for_each(|(curr, remove)| {
                *curr -= remove;
            });
        self.len = self.counts.iter().sum();
    }
    /// Adds tiles into `self`.
    pub fn insert<I>(&mut self, tiles: I)
    where
        I: Iterator<Item = Tile>,
    {
        let counts = Self::counts(tiles);

        self.counts.iter_mut().zip(counts).for_each(|(curr, add)| {
            *curr += add;
        });
        self.len = self.counts.iter().sum();
    }
    /// Helper method to get an array of counts for each tile in
    /// the iterator.
    fn counts<I>(tiles: I) -> [usize; 27]
    where
        I: Iterator<Item = Tile>,
    {
        let mut counts = [0; 27];
        for t in tiles {
            counts[usize::from(t)] += 1;
        }
        counts
    }
}
impl FromIterator<Tile> for TileCounts {
    fn from_iter<T: IntoIterator<Item = Tile>>(tiles: T) -> Self {
        Self::from(Self::counts(tiles.into_iter()))
    }
}
impl From<[usize; 27]> for TileCounts {
    fn from(counts: [usize; 27]) -> Self {
        let len = counts.iter().sum();

        Self { counts, len }
    }
}
