//! Models the scrabble board.

use crate::{
    error::{GameError, GameResult},
    game::{play::PlaceBuilder, tile::Tile},
    util::{self, bitboard::BitBoard, fsm::Fsm, pos::Pos, scoring, words::WordsIteratorExt},
};
use std::fmt;

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
            self.board.grid[usize::from(pos)] = Some(tile);
            self.board.occ_h.set(pos);
            self.board.occ_v.set(pos.anti_clockwise90());
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
#[derive(Clone, Debug)]
pub struct Board {
    grid: [Option<Tile>; CELLS],
    /// regular occupancy, for finding horizontal words.
    occ_h: BitBoard,
    /// vertical occupancy, rotated 90deg. For finding vertical words.
    occ_v: BitBoard,
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
        util::write_grid(f, |pos| match self.get(pos) {
            Some(tile) => format!("{}", tile),
            None => " . ".to_string(),
        })
    }
}
impl Board {
    /// Computes the combined score for horizontal and vertical words, adding
    /// the 50 point bonus where appropriate. `new` is the (horizontal) bitboard
    /// of added tiles. If an invalid word is encountered, returns an error.
    fn score_and_validate<'a>(
        &self,
        new_h: BitBoard,
        new_v: BitBoard,
        fsm: &impl Fsm<'a>,
    ) -> GameResult<usize> {
        let words_h = self
            .occ_h
            .word_boundaries()
            .intersecting(new_h)
            .horizontal();
        let words_v = self.occ_v.word_boundaries().intersecting(new_v).vertical();

        for w in self.occ_v.word_boundaries().intersecting(new_v).vertical() {
            println!("{w:?}");
        }

        let mut score = 0;
        for word in words_h.chain(words_v) {
            println!("{word:?}");
            for pos in word {
                println!(
                    "{pos} {:?} {}",
                    pos.premium(),
                    self.grid[usize::from(pos)].unwrap().score()
                );
            }

            println!("  score: {}", scoring::score(word, &new_h, self, fsm)?);

            score += scoring::score(word, &new_h, self, fsm)?;
        }

        // If the bitcount for `new_h` is 7, add a 50 point bonus.
        match new_h.bit_count() {
            7 => Ok(score + 50),
            _ => Ok(score),
        }
    }
    /// Gets the board occupancy.
    pub fn occ_h(&self) -> &BitBoard {
        &self.occ_h
    }
    /// Gets the rotated board occupancy.
    pub fn occ_v(&self) -> &BitBoard {
        &self.occ_v
    }
    /// Gets the tile at `pos`.
    pub fn get(&self, pos: impl Into<Pos>) -> Option<Tile> {
        self.grid[usize::from(pos.into())]
    }
    /// Removes all tiles in `tile_positions` from the board.
    pub fn undo_placement(&mut self, tile_positions: &[(Pos, Tile)]) {
        for &(pos, _) in tile_positions {
            self.grid[usize::from(pos)] = None;
            self.occ_h.clear(pos);
            self.occ_v.clear(pos.anti_clockwise90());
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
        let (row, col) = tile_positions[0].0.row_col();
        let mut same_row = true;
        let mut same_col = true;

        // new tiles for horizontal words
        let mut new_h = BitBoard::default();
        // new tiles for vertical words: rotated 90deg anticlockwise
        let mut new_v = BitBoard::default();

        for &(pos_h, _) in tile_positions {
            // if the bit has already been set then `tile_positions` contains
            // a duplicate tile.
            if new_h.is_set(pos_h) {
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
        util::validate_occ_h(self.occ_h, new_h)?;

        // update bitboards
        self.occ_h |= new_h;
        self.occ_v |= new_v;

        // Tiles positions have now been validated: place the tiles on the board.
        // Word validation requires that these tiles are present. If an invalid
        // word exists on the board, the tiles will be removed.
        for &(pos, tile) in tile_positions {
            self.grid[usize::from(pos)] = Some(tile);
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
