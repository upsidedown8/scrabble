//! Models the scrabble Rack.

use crate::{
    error::{GameError, GameResult},
    game::{letter_bag::LetterBag, tile::Tile},
    util::tile_counts::TileCounts,
};
use std::fmt;

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
    /// Creates a new [`Rack`] with the provided tiles.
    pub fn new_with_tiles(tiles: &[Tile]) -> Self {
        Self {
            counts: TileCounts::from_iter(tiles.iter().take(7).copied()),
        }
    }
    /// Get the underlying tile counts for the rack.
    pub fn tile_counts(&self) -> &TileCounts {
        &self.counts
    }
    /// Gets the sum of the remaining tiles on the rack. This is used
    /// for scoring at the end of the game.
    pub fn tile_sum(&self) -> usize {
        self.counts.tile_sum()
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
        // check number of tiles
        if !(1..=7).contains(&tiles.len()) {
            return Err(GameError::RedrawCount);
        }

        // check whether there are enough new letters in the bag
        if letter_bag.len() < tiles.len() {
            return Err(GameError::NotEnoughLetters);
        }

        // check whether all tiles are within the rack
        if !self.counts.contains(tiles.iter().copied()) {
            return Err(GameError::NotInRack);
        }

        // remove current tiles & draw new tiles
        self.counts.remove(tiles.iter().copied());
        self.counts.insert(letter_bag.draw_many(tiles.len()));

        // add removed tiles back into bag
        letter_bag.add_tiles(tiles.iter().copied());

        Ok(())
    }
    /// Checks whether all `tiles` are contained within the rack.
    pub fn contains(&self, tiles: impl Iterator<Item = Tile>) -> bool {
        self.counts.contains(tiles)
    }
    /// Removes all `tiles` from the rack.
    pub fn remove(&mut self, tiles: impl Iterator<Item = Tile>) {
        self.counts.remove(tiles)
    }
    /// Gets an iterator over the tiles in the rack.
    pub fn iter(&self) -> impl Iterator<Item = Tile> + '_ {
        self.counts.iter()
    }
}
