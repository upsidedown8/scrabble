//! Models the [`LetterBag`].

use crate::{
    game::{rack::RACK_SIZE, tile::Tile},
    util::tile_counts::TileCounts,
};
use rand::Rng;
use std::iter::once;

/// A structure containing a finite number of tiles which can
/// be used during the game. Since there are 27 tiles, an array
/// with 27 elements is used to keep count.
#[derive(Debug)]
pub struct LetterBag {
    counts: TileCounts,
}

impl Default for LetterBag {
    fn default() -> Self {
        // Uses the initial counts from the official game.
        let mut counts = [0; 27];
        for (idx, tile) in Tile::iter().enumerate() {
            counts[idx] = Self::initial_count(tile);
        }

        Self::from(counts)
    }
}
impl From<[usize; 27]> for LetterBag {
    fn from(counts: [usize; 27]) -> Self {
        Self {
            counts: TileCounts::from(counts),
        }
    }
}
impl LetterBag {
    /// Checks whether the bag is empty.
    pub fn is_empty(&self) -> bool {
        self.counts.is_empty()
    }
    /// Returns the total number of tiles remaining in the bag.
    pub fn len(&self) -> usize {
        self.counts.len()
    }
    /// Gets the underlying tile counts.
    pub fn counts(self) -> TileCounts {
        self.counts
    }
    /// Gets the initial count for `tile` in the official version
    /// of scrabble.
    pub fn initial_count(tile: Tile) -> usize {
        const INIT_COUNTS: [usize; 27] = [
            9,  // A
            2,  // B
            2,  // C
            4,  // D
            12, // E
            2,  // F
            3,  // G
            2,  // H
            9,  // I
            1,  // J
            1,  // K
            4,  // L
            2,  // M
            6,  // N
            8,  // O
            2,  // P
            1,  // Q
            6,  // R
            4,  // S
            6,  // T
            4,  // U
            2,  // V
            2,  // W
            1,  // X
            2,  // Y
            1,  // Z
            2,  // Blank
        ];

        INIT_COUNTS[usize::from(tile)]
    }
    /// Dras a randomly selected letter from the bag.
    /// Returns [`None`]
    pub fn draw(&mut self) -> Option<Tile> {
        match self.len() {
            0 => None,
            len => Some({
                // Generate a random index, as though all tiles
                // in the bag were layed out in a single array.
                let idx = rand::thread_rng().gen_range(0..len);
                // traverse the tiles until `idx` is reached
                let mut tile_idx = 0;
                let mut count = self.counts.count(tile_idx);

                // idx + 1 is the number of tiles in where the chosen card
                // exists, so use <= instead of <.
                while count <= idx {
                    tile_idx += 1;
                    count += self.counts.count(tile_idx);
                }

                // since `idx` < `total`, this assertion should never fail
                assert!(tile_idx < 27);

                let tile = Tile::from(tile_idx);

                // decrement the count for the chosen tile, and the overall total
                self.counts.remove(once(tile));

                tile
            }),
        }
    }
    /// Draws `min(count, total, RACK_SIZE)` tiles from the bag as an
    /// iterator. ie. At most [`RACK_SIZE`] tiles, no more than the total
    /// number of tiles, and no more than `count` tiles.
    pub fn draw_many(&mut self, count: usize) -> impl Iterator<Item = Tile> + '_ {
        (0..RACK_SIZE).filter_map(|_| self.draw()).take(count)
    }
    /// Adds up to [`RACK_SIZE`] tiles from the provided iterator back into
    /// the bag, returning the number of tiles that were added.
    pub fn add_tiles(&mut self, tiles: impl Iterator<Item = Tile>) -> usize {
        let len = self.len();
        self.counts.insert(tiles.take(RACK_SIZE));
        self.len() - len
    }
}

#[cfg(test)]
mod tests {
    use crate::game::{letter_bag::LetterBag, rack::RACK_SIZE, tile::Tile};

    #[test]
    fn draw_limits() {
        let mut letter_bag = LetterBag::default();
        assert_eq!(letter_bag.draw_many(0).count(), 0);
        assert_eq!(letter_bag.draw_many(100).count(), RACK_SIZE);
        assert_eq!(letter_bag.len(), 93);
    }

    #[test]
    fn empty_bag() {
        let mut letter_bag = LetterBag::default();
        let mut counts = [0; 27];
        let mut removed = vec![];

        while !letter_bag.is_empty() {
            for tile in letter_bag.draw_many(RACK_SIZE) {
                counts[usize::from(tile)] += 1;
                removed.push(tile);
            }
        }

        for (i, &item) in counts.iter().enumerate() {
            assert_eq!(item, LetterBag::initial_count(Tile::from(i)));
        }

        let mut len = 0;
        while len < 100 {
            letter_bag.add_tiles(removed[len..].iter().copied().take(7));

            len = (len + RACK_SIZE).min(100);
        }

        assert_eq!(letter_bag.len(), len);
    }
}
