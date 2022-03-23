//! Contains a structure for keeping track of how many of each
//! of the 27 tiles are in a container.

use crate::game::tile::Tile;
use std::iter::repeat;

/// Reusable structure used to store a quantity of each tile.
#[derive(Default, Debug, Clone, Copy)]
pub struct TileCounts {
    counts: [usize; 27],
    len: usize,
}

impl TileCounts {
    /// Gets the sum of the tile values.
    pub fn tile_sum(&self) -> usize {
        Tile::iter()
            .zip(self.counts.iter())
            .map(|(tile, &count)| tile.score() * count)
            .sum()
    }
    /// The number of tiles in `self`.
    pub fn len(&self) -> usize {
        self.len
    }
    /// Checks whether the counts are empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Gets the count for a specific tile.
    pub fn count(&self, tile: impl Into<Tile>) -> usize {
        self.counts[usize::from(tile.into())]
    }
    /// Checks whether there are any of a tile.
    pub fn any(&self, tile: impl Into<Tile>) -> bool {
        self.count(tile) > 0
    }

    /// Insert a single tile.
    pub fn insert_one(&mut self, tile: impl Into<Tile>) {
        self.counts[usize::from(tile.into())] += 1;
        self.len += 1;
    }
    /// Remove a single tile.
    pub fn remove_one(&mut self, tile: impl Into<Tile>) {
        self.counts[usize::from(tile.into())] -= 1;
        self.len -= 1;
    }

    /// An iterator over the tiles in `self`.
    pub fn iter(&self) -> impl Iterator<Item = Tile> + '_ {
        self.counts
            .iter()
            .enumerate()
            .flat_map(|(tile, &count)| repeat(Tile::from(tile)).take(count))
    }
    /// An iterator over the distinct tiles in `self`.
    pub fn iter_unique(&self) -> impl Iterator<Item = Tile> + '_ {
        self.counts
            .iter()
            .filter(|&&count| count > 0)
            .enumerate()
            .map(|(idx, _)| Tile::from(idx))
    }

    /// Checks whether the `tiles` are contained within the counts.
    pub fn contains(&self, tiles: impl IntoIterator<Item = Tile>) -> bool {
        Self::counts(tiles)
            .into_iter()
            .zip(self.counts)
            .all(|(a, b)| a <= b)
    }
    /// Removes tiles from `self`. `self` should have sufficient tiles
    /// otherwise this method may panic.
    pub fn remove(&mut self, tiles: impl IntoIterator<Item = Tile>) {
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
    pub fn insert(&mut self, tiles: impl IntoIterator<Item = Tile>) {
        let counts = Self::counts(tiles);

        self.counts.iter_mut().zip(counts).for_each(|(curr, add)| {
            *curr += add;
        });
        self.len = self.counts.iter().sum();
    }

    /// Helper method to get an array of counts for each tile in
    /// the iterator.
    fn counts(tiles: impl IntoIterator<Item = Tile>) -> [usize; 27] {
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
impl From<TileCounts> for [usize; 27] {
    fn from(TileCounts { counts, .. }: TileCounts) -> Self {
        counts
    }
}

#[cfg(test)]
mod tests {
    use crate::game::tile::Letter;

    use super::*;

    fn tile(num: usize) -> Tile {
        Tile::Letter(Letter::from(num))
    }

    #[test]
    fn insert_contains() {
        let mut tc = TileCounts::default();

        tc.insert([tile(0), tile(0), tile(1), tile(2), tile(2), tile(25)]);

        assert!(tc.contains([tile(0), tile(0)]));
        assert!(!tc.contains([tile(0), tile(0), tile(0)]));
    }

    #[test]
    fn counts() {
        let mut tc = TileCounts::default();

        tc.insert([tile(0), tile(1), tile(2), tile(10), tile(9), tile(0)]);

        assert_eq!(2, tc.count(tile(0)));
        assert_eq!(1, tc.count(tile(1)));
        assert_eq!(1, tc.count(tile(2)));
        assert_eq!(1, tc.count(tile(9)));
        assert_eq!(1, tc.count(tile(10)));
    }

    #[test]
    fn iter() {
        let mut tc = TileCounts::default();

        tc.insert([
            tile(0),
            tile(1),
            tile(2),
            tile(2),
            tile(0),
            tile(7),
            tile(20),
            tile(20),
        ]);

        // should traverse in sorted order.
        let order = [0, 0, 1, 2, 2, 7, 20, 20];
        assert!(tc
            .iter()
            .zip(order)
            .all(|(a, b)| usize::from(a.letter().unwrap()) == b));
    }

    #[test]
    fn remove() {
        let mut tc = TileCounts::default();

        tc.insert([tile(0), tile(0), tile(1), tile(2)]);

        assert_eq!(2, tc.count(tile(0)));
        tc.remove([tile(0)]);
        assert_eq!(1, tc.count(tile(0)));
        tc.remove([tile(0)]);
        assert_eq!(0, tc.count(tile(0)));

        assert_eq!(tc.len(), 2);
    }
}
