//! Models the scrabble board.

use crate::{
    error::{GameError, GameResult},
    game::tile::Tile,
    util::{
        self,
        bitboard::BitBoard,
        fsm::Fsm,
        pos::Pos,
        scoring,
        words::{Word, Words},
    },
};
use std::fmt;

/// The number of rows on the board.
pub const ROWS: usize = 15;
/// The number of columns on the board.
pub const COLS: usize = 15;
/// The number of squares on the board.
pub const CELLS: usize = 15 * 15;

/// Represents the 15 x 15 scrabble board, storing the location of
/// tiles, and allowing [`Play`](super::play::Play)s to be made
/// and validated.
#[derive(Clone, Debug)]
pub struct Board {
    grid: [Option<Tile>; CELLS],
    /// regular occupancy, for finding horizontal words.
    occ_h: BitBoard,
    /// vertical occupancy, rotated 90deg. For finding vertical words.
    occ_v: BitBoard,
}
impl Board {
    /// Computes the combined score for horizontal and vertical words, adding
    /// the 50 point bonus where appropriate.
    ///
    /// Finds the sum of the scores of each word. If an invalid word is
    /// encountered, returns an error, otherwise returns the sum of the
    /// scores of all words containing new letters. `map_pos` is used to rotate
    /// the bit positions back to the standard grid for the rotated vertical
    /// bitboard. `new` is the set of added tiles, which should ave been
    /// rotated previously for vertical words. `occ` is the set of existing
    /// tiles, which should also have been rotated.
    fn score_and_validate<'a>(
        &self,
        new_h: BitBoard,
        new_v: BitBoard,
        fsm: &impl Fsm<'a>,
    ) -> GameResult<usize> {
        let words_h = Words::horizontal(self.occ_h | new_h);
        let words_v = Words::vertical(self.occ_v | new_v);

        let mut score = 0;
        for word in words_h.chain(words_v) {
            score += scoring::score(word, &new_h, self, fsm)?;
        }

        // If the bitcount for `new_h` is 7, add a 50 point bonus.
        match new_h.bit_count() {
            7 => Ok(score + 50),
            _ => Ok(score),
        }
    }
    /// Gets an iterator over the words on the board.
    pub fn words(&self) -> impl Iterator<Item = Word> {
        let words_h = Words::horizontal(self.occ_h);
        let words_v = Words::vertical(self.occ_v);

        words_h.chain(words_v)
    }
    /// Gets the board occupancy.
    pub fn occ_h(&self) -> &BitBoard {
        &self.occ_h
    }
    /// Gets the rotated board occupancy.
    pub fn occ_v(&self) -> &BitBoard {
        &self.occ_v
    }
    /// Gets the tile at `pos`
    pub fn at(&self, pos: impl Into<Pos>) -> Option<Tile> {
        self.grid[usize::from(pos.into())]
    }
    /// Removes all tiles in `tile_positions` from the board.
    pub fn undo_placement(&mut self, tile_positions: Vec<Pos>) {
        for pos in tile_positions {
            self.grid[usize::from(pos)] = None;
            self.occ_h.clear_bit(pos);
            self.occ_v.clear_bit(pos.anti_clockwise90());
        }
    }
    /// Attempts to perform a [`Play::Place`](super::play::Play::Place)
    /// on the board. (All other variants don't require board modification).
    /// If succesful, returns the score from placing the new tiles.
    pub fn make_placement<'a>(
        &mut self,
        tile_positions: &[(Pos, Tile)],
        fsm: &impl Fsm<'a>,
    ) -> GameResult<usize> {
        // new tiles for horizontal words
        let mut new_h = BitBoard::default();
        // new tiles for vertical words: rotated 90deg anticlockwise
        let mut new_v = BitBoard::default();

        for &(pos_h, _) in tile_positions {
            // if the bit has already been set then `tile_positions` contains
            // a duplicate tile.
            if new_h.is_bit_set(pos_h) {
                return Err(GameError::DuplicatePosition);
            }

            new_h.set_bit(pos_h);
            new_v.set_bit(pos_h.anti_clockwise90());
        }

        // perform tile placement validation
        util::validate_occ_h(self.occ_h, new_h)?;

        // Tiles positions have now been validated: place the tiles on the board.
        // Word validation requires that these tiles are present. If an invalid
        // word exists on the board, the tiles will be removed.
        for &(pos, tile) in tile_positions {
            self.grid[usize::from(pos)] = Some(tile);
        }

        // checks that words are valid then returns the score
        match self.score_and_validate(new_h, new_v, fsm) {
            // everything was ok, update the bitboards.
            Ok(score) => {
                // update bitboards
                self.occ_h |= new_h;
                self.occ_v |= new_v;

                Ok(score)
            }
            // error occured, reverse the state change
            Err(e) => {
                // clear all modified squares
                tile_positions
                    .iter()
                    .for_each(|(pos, _)| self.grid[usize::from(*pos)] = None);

                Err(e)
            }
        }
    }
}
impl Default for Board {
    fn default() -> Self {
        Self {
            grid: [None; CELLS],
            occ_h: BitBoard::default(),
            occ_v: BitBoard::default(),
        }
    }
}
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        util::write_grid(f, |pos| match self.at(pos) {
            Some(tile) => format!("{}", tile),
            None => " . ".to_string(),
        })
    }
}
