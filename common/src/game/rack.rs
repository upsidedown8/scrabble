//! Models the scrabble Rack.

use crate::game::{letter_bag::LetterBag, tile::Tile};
use std::fmt;

/// The maximum number of tiles that can be stored
/// on a player's rack.
pub const RACK_SIZE: usize = 7;

/// Each player has a rack with up to 7 tiles on it.
/// The rack is modelled as a vector containing up
/// to 7 [`tiles`](Tile).
#[derive(Debug)]
pub struct Rack {
    tiles: Vec<Tile>,
}

impl fmt::Display for Rack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for tile in self.tiles.iter() {
            write!(f, "{}", tile)?;
        }
        write!(f, "]")
    }
}
impl Rack {
    /// Creates a new [`Rack`], drawing [`RACK_SIZE`] letters
    /// from `letter_bag`.
    pub fn new(letter_bag: &mut LetterBag) -> Self {
        let tiles: Vec<_> = letter_bag.draw_many(RACK_SIZE).collect();
        assert_eq!(tiles.len(), RACK_SIZE);

        Self { tiles }
    }
    /// Adds tiles from `letter_bag` to attempt to increase the
    /// number of tiles in the rack to [`RACK_SIZE`].
    pub fn fill(&mut self, letter_bag: &mut LetterBag) {
        self.tiles
            .extend(letter_bag.draw_many(self.missing_count()))
    }
    /// Gets the number of tiles below [`RACK_SIZE`] in the rack.
    pub fn missing_count(&self) -> usize {
        RACK_SIZE - self.len()
    }
    /// Gets the number of tiles in the rack.
    pub fn len(&self) -> usize {
        self.tiles.len()
    }
    /// Checks whether the rack is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Exchanges the tiles provided with new ones in the `letter_bag`.
    /// If there are insufficient tiles in the `letter_bag`, or the tiles
    /// provided are not all present in the rack, returns [`None`].
    pub fn exchange_tiles(&self, tiles: Vec<Tile>, letter_bag: &mut LetterBag) -> Option<Self> {
        todo!()
    }
    /// Gets an iterator over the tiles in the rack.
    pub fn iter(&self) -> impl Iterator<Item = Tile> + '_ {
        self.tiles.iter().copied()
    }
}
