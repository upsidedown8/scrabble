use rand::Rng;

use super::tile::Tile;

/// A structure containing a finite number of tiles which can
/// be used during the game. Since there are 27 tiles, an array
/// with 27 elements is used to keep count.
pub struct LetterBag {
    /// Count for each tile
    counts: [usize; 27],
    /// Total of `counts`
    total: usize,
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
            counts,
            total: counts.iter().sum(),
        }
    }
}
impl LetterBag {
    /// Checks whether the bag is empty.
    pub fn is_empty(&self) -> bool {
        self.total() == 0
    }
    /// Gets the number of a specific tile remaining in the bag.
    pub fn count(&self, tile: Tile) -> usize {
        self.counts[usize::from(tile)]
    }
    /// Returns the total number of tiles remaining in the bag.
    pub fn total(&self) -> usize {
        self.total
    }
    /// Returns an iterator over each tile and its corresponding
    /// count.
    pub fn iter(&self) -> impl Iterator<Item = (Tile, usize)> + '_ {
        Tile::iter().map(|tile| (tile, self.count(tile)))
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
        match self.total() {
            0 => None,
            total => Some({
                // Generate a random index, as though all tiles
                // in the bag were layed out in a single array.
                let idx = rand::thread_rng().gen_range(0..total);
                // traverse the tiles until `idx` is reached
                let mut count = self.count(Tile::from(0));
                let mut tile_idx = 0;

                // idx + 1 is the number of tiles in where the chosen card
                // exists, so use <= instead of <.
                while count <= idx {
                    tile_idx += 1;
                    count += self.count(Tile::from(tile_idx));
                }

                // since `idx` < `total`, this assertion should never fail
                assert!(tile_idx < 27);

                // decrement the count for the chosen tile, and the overall total
                self.counts[tile_idx] -= 1;
                self.total -= 1;

                Tile::from(tile_idx)
            }),
        }
    }
    /// Draws `min(count, total, 7)` tiles from the bag as an
    /// iterator. ie. At most 7 tiles, no more than the total
    /// number of tiles, and no more than `count` tiles.
    pub fn draw_many(&mut self, count: usize) -> impl Iterator<Item = Tile> + '_ {
        (0..7).filter_map(|_| self.draw()).take(count)
    }
    /// Adds up to 7 tiles from the provided iterator back into
    /// the bag, returning the number of tiles that were added.
    pub fn add_tiles(&mut self, tiles: impl Iterator<Item = Tile>) -> usize {
        (0..7)
            .zip(tiles)
            .map(|(_, tile)| {
                self.counts[usize::from(tile)] += 1;
                self.total += 1;
            })
            .count()
    }
}

#[cfg(test)]
mod tests {
    use crate::game::tile::Tile;

    use super::LetterBag;

    #[test]
    fn draw_limits() {
        let mut letter_bag = LetterBag::default();
        assert_eq!(letter_bag.draw_many(0).count(), 0);
        assert_eq!(letter_bag.draw_many(100).count(), 7);
        assert_eq!(letter_bag.total(), 93);
    }

    #[test]
    fn empty_bag() {
        let mut letter_bag = LetterBag::default();
        let mut counts = [0; 27];
        let mut removed = vec![];

        while !letter_bag.is_empty() {
            for tile in letter_bag.draw_many(7) {
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

            len = (len + 7).min(100);
        }

        assert_eq!(letter_bag.total(), len);
    }
}
