//! Models the scrabble board.

use crate::{
    error::{GameError, GameResult},
    game::{play::PlaceBuilder, tile::Tile},
    util::{self, bitboard::BitBoard, fsm::Fsm, grid::Grid, pos::Pos, scoring, words::WordsExt},
};
use std::{fmt, ops::Index};

/// The number of rows on the board.
pub const ROWS: usize = 15;
/// The number of columns on the board.
pub const COLS: usize = 15;
/// The number of squares on the board.
pub const CELLS: usize = ROWS * COLS;

/// Used to construct an arbitrary board position without validation.
#[derive(Debug, Default)]
pub struct BoardBuilder {
    board: Board,
}
impl BoardBuilder {
    /// Places a word on the board, without performing any validation checks.
    pub fn place(mut self, builder: PlaceBuilder) -> Self {
        for (pos, tile) in builder.tile_positions(&self.board) {
            self.board.set(pos, Some(tile));
        }
        self
    }
    /// Constructs the [`Board`].
    pub fn build(self) -> Board {
        self.board
    }
}

/// Represents the 15 x 15 scrabble board, storing the location of
/// tiles, and allowing [`Play`](super::play::Play)s to be made
/// and validated.
#[derive(Clone, Debug, Default)]
pub struct Board {
    grid_h: Grid,
    grid_v: Grid,
}
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        util::write_grid(f, |pos| match self[pos] {
            Some(tile) => format!("{}", tile),
            None => " . ".to_string(),
        })
    }
}
impl<T: Into<Pos>> Index<T> for Board {
    type Output = Option<Tile>;

    fn index(&self, index: T) -> &Self::Output {
        self.grid_h().index(index.into())
    }
}
impl Board {
    /// Computes the combined score for horizontal and vertical words, adding
    /// the 50 point bonus where appropriate. If an invalid word is encountered,
    /// returns an error.
    fn score_and_validate<'a>(
        &self,
        new_h: BitBoard,
        new_v: BitBoard,
        fsm: &impl Fsm<'a>,
    ) -> GameResult<usize> {
        let mut score = 0;

        // find and score the horizontal words.
        let occ_h = self.grid_h.occ();
        let words_h = occ_h
            .word_boundaries()
            .intersecting(new_h)
            .words(&self.grid_h);
        for word in words_h {
            score += scoring::score(word, &new_h, fsm)?;
        }

        // find and score the vertical words.
        let occ_v = self.grid_v.occ();
        let words_v = occ_v
            .word_boundaries()
            .intersecting(new_v)
            .words(&self.grid_v);
        for word in words_v {
            score += scoring::score(word, &new_v, fsm)?;
        }

        // If the bitcount for `new_h` is 7, add a 50 point bonus.
        match new_h.bit_count() {
            7 => Ok(score + 50),
            _ => Ok(score),
        }
    }
    /// Sets the tile at `pos`.
    fn set(&mut self, pos: Pos, tile: impl Into<Option<Tile>>) {
        self.grid_h.set(pos, tile);
        self.grid_v.set(pos.anti_clockwise90(), tile);
    }
    /// Gets the board occupancy.
    pub fn grid_h(&self) -> &Grid {
        &self.grid_h
    }
    /// Gets the rotated board occupancy.
    pub fn grid_v(&self) -> &Grid {
        &self.grid_v
    }
    /// Removes all tiles in `tile_positions` from the board.
    pub fn undo_placement(&mut self, tile_positions: &[(Pos, Tile)]) {
        for &(pos, _) in tile_positions {
            self.set(pos, None);
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
        // check the tile count
        if !(1..=7).contains(&tile_positions.len()) {
            return Err(GameError::PlacementCount);
        }

        // store the row and column of the first tile.
        let (first_pos, _) = tile_positions[0];
        let (row, col) = first_pos.row_col();
        let mut same_row = true;
        let mut same_col = true;

        // new tiles for horizontal words
        let mut new_h = BitBoard::default();
        // new tiles for vertical words: rotated 90deg anticlockwise
        let mut new_v = BitBoard::default();

        for &(pos_h, _) in tile_positions {
            // if the bit has already been set then `tile_positions` contains
            // a duplicate tile.
            if new_h[pos_h] {
                return Err(GameError::DuplicatePosition);
            }

            // compare row and col with the first row.
            same_row &= row == pos_h.row();
            same_col &= col == pos_h.col();

            new_h.set(pos_h);
            new_v.set(pos_h.anti_clockwise90());
        }

        if !same_row && !same_col {
            return Err(GameError::NoCommonLine);
        }

        // perform tile placement validation
        let &occ_h = self.grid_h.occ();
        util::validate_occ_h(occ_h, new_h)?;

        // Tiles positions have now been validated: place the tiles on the board.
        // Word validation requires that these tiles are present. If an invalid
        // word exists on the board, the tiles will be removed.
        for &(pos, tile) in tile_positions {
            self.set(pos, tile);
        }

        // checks that words are valid then returns the score
        match self.score_and_validate(new_h, new_v, fsm) {
            // everything was ok, update the bitboards.
            Ok(score) => Ok(score),
            // error occured, reverse the state change
            Err(e) => {
                self.undo_placement(tile_positions);
                Err(e)
            }
        }
    }
}
