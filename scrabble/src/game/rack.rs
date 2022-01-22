//! Models the scrabble Rack.

use crate::game::{
    error::{GameError, GameResult},
    letter_bag::LetterBag,
    tile::Tile,
};
use std::{fmt, iter::repeat};

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

/// The maximum number of tiles that can be stored
/// on a player's rack.
pub const RACK_SIZE: usize = 7;

/// Each player has a rack with up to 7 tiles on it.
/// The rack is modelled as a vector containing up
/// to 7 [`tiles`](Tile).
#[derive(Debug)]
pub struct Rack {
    counts: TileCounts,
}

impl fmt::Display for Rack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for tile in self.iter() {
            write!(f, "{}", tile)?;
        }
        write!(f, "]")
    }
}
impl Rack {
    /// Creates a new [`Rack`], drawing [`RACK_SIZE`] letters
    /// from `letter_bag`.
    pub fn new(letter_bag: &mut LetterBag) -> Self {
        let counts = TileCounts::from_iter(letter_bag.draw_many(RACK_SIZE));

        assert_eq!(counts.len(), RACK_SIZE);

        Self { counts }
    }
    /// Adds tiles from `letter_bag` to attempt to increase the
    /// number of tiles in the rack to [`RACK_SIZE`].
    pub fn refill(&mut self, letter_bag: &mut LetterBag) {
        self.counts
            .insert(letter_bag.draw_many(self.missing_count()));
    }
    /// Gets the number of tiles below [`RACK_SIZE`] in the rack.
    pub fn missing_count(&self) -> usize {
        RACK_SIZE - self.len()
    }
    /// Gets the number of tiles in the rack.
    pub fn len(&self) -> usize {
        self.counts.len()
    }
    /// Checks whether the rack is empty.
    pub fn is_empty(&self) -> bool {
        self.counts.is_empty()
    }
    /// Exchanges the tiles provided with new ones in the `letter_bag`.
    /// If there are insufficient tiles in the `letter_bag`, or the tiles
    /// provided are not all present in the rack, returns [`None`].
    pub fn exchange_tiles(&mut self, tiles: &[Tile], letter_bag: &mut LetterBag) -> GameResult<()> {
        // check whether all tiles are within the rack
        if !self.counts.contains(tiles.iter().copied()) {
            return Err(GameError::NotInRack);
        }

        // check whether there are enough new letters in the bag
        if letter_bag.len() < tiles.len() {
            return Err(GameError::NotEnoughLetters);
        }

        // remove current tiles & draw new tiles
        self.counts.remove(tiles.iter().copied());
        self.counts.insert(letter_bag.draw_many(tiles.len()));

        // add removed tiles back into bag
        letter_bag.add_tiles(tiles.iter().copied());

        Ok(())
    }
    /// Checks whether all `tiles` are contained within the rack.
    pub fn contains<I>(&self, tiles: I) -> bool
    where
        I: Iterator<Item = Tile>,
    {
        self.counts.contains(tiles)
    }
    /// Removes all `tiles` from the rack.
    pub fn remove<I>(&mut self, tiles: I)
    where
        I: Iterator<Item = Tile>,
    {
        self.counts.remove(tiles)
    }
    /// Gets an iterator over the tiles in the rack.
    pub fn iter(&self) -> impl Iterator<Item = Tile> + '_ {
        self.counts.iter()
    }
}
