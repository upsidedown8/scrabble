//! Models the scrabble board.

use crate::game::{
    bitboard::BitBoard,
    bitboard::Bits,
    error::{GameError, GameResult},
    pos::{Col, Pos, Row},
    tile::Tile,
    word_tree::WordTree,
};
use std::fmt;

/// The number of rows on the board.
pub const ROWS: usize = 15;
/// The number of columns on the board.
pub const COLS: usize = 15;
/// The number of squares on the board.
pub const CELLS: usize = 15 * 15;

/// Struct wrapping the result of `Board::word_boundaries` which
/// implements the `Iterator` trait, going over tuples containing
/// the start and end of each word (inclusive).
pub struct WordBoundaries(Bits);
impl WordBoundaries {
    /// Creates a new `WordBoundaries` instance, going over boundaries
    /// calculated from `occ`. This assumes that words in `occ` always
    /// read left to right so for vertical words, the occupancy must
    /// first be rotated 90degrees anticlockwise.
    pub fn new(occ: BitBoard) -> Self {
        Self(Self::word_boundaries(occ).into_iter())
    }

    /// Calculates the set of tiles which start a word.
    ///
    /// In general, a letter is the start of a word if, in its
    /// row or column (depending on direction of the word), it
    /// is (preceded by an empty square OR is an edge square)
    /// AND (is succeeded by a non-empty square in the same row
    /// or column).
    ///
    /// This function is also used for vertical word boundaries,
    /// but the vertical occupancy is rotated by 90deg anticlockwise
    /// so that vertical words read left to right. A single bitwise
    /// traversal can then be used to find all words.
    pub fn word_boundaries(occ: BitBoard) -> BitBoard {
        // finds all squares which start a horizontal word
        let horizontal_starts = (occ << 1) & !(occ >> 1) & !BitBoard::rightmost_col();
        let horizontal_ends = (occ >> 1) & !(occ << 1) & !BitBoard::leftmost_col();

        // simplified with boolean algebra:
        // (occ & horizontal_starts) | (occ & vertical_starts)
        // = occ & (horizontal_starts | vertical_starts)
        occ & (horizontal_starts | horizontal_ends)
    }
}
impl Iterator for WordBoundaries {
    type Item = WordBoundary;

    fn next(&mut self) -> Option<WordBoundary> {
        match (self.0.next(), self.0.next()) {
            (Some(start), Some(end)) => Some(WordBoundary::new(start, end)),
            _ => None,
        }
    }
}

/// The start and end of a specific word.
#[derive(Clone, Copy, Debug)]
pub struct WordBoundary {
    start: Pos,
    end: Pos,
}
impl WordBoundary {
    /// Creates a new `WordBoundary`. `start` should be less than `end.
    pub fn new(start: Pos, end: Pos) -> Self {
        debug_assert!(start < end);

        Self { start, end }
    }
    /// An iterator between the `start` and `end` positions, inclusive
    pub fn iter_range(&self) -> impl Iterator<Item = Pos> {
        let start = usize::from(self.start);
        let end = usize::from(self.end);

        (start..=end).map(Pos::from)
    }
    /// Gets the start position
    pub fn start(&self) -> Pos {
        self.start
    }
    /// Gets the end position
    pub fn end(&self) -> Pos {
        self.end
    }
    /// Checks whether `pos` fits within the inclusive range,
    /// `start` <= `pos` <= `end`.
    pub fn contains(&self, pos: Pos) -> bool {
        self.start <= pos && pos <= self.end
    }
}

/// Represents the 15 x 15 scrabble board, storing the location of
/// tiles, and allowing [`Play`]s to be made and validated.
#[derive(Clone, Debug)]
pub struct Board {
    grid: [Option<Tile>; CELLS],
    /// regular occupancy, for finding horizontal words.
    occupancy_h: BitBoard,
    /// vertical occupancy, rotated 90deg. For finding vertical words.
    occupancy_v: BitBoard,
}
impl Board {
    /// Rotates a `pos` 90 degrees anticlockwise about the center square.
    fn anti_clockwise_90(pos: Pos) -> Pos {
        let (r, c) = pos.cartesian();

        let r_prime = Row::from(14 - usize::from(c));
        let c_prime = Col::from(usize::from(r));

        Pos::from((r_prime, c_prime))
    }
    /// Rotates a `pos` 90 degrees clockwise about the center square. Inverse
    /// functon of `rotate_90_anti_clockwise`.
    fn clockwise_90(pos: Pos) -> Pos {
        let (r, c) = pos.cartesian();

        let r_prime = Row::from(14 - usize::from(c));
        let c_prime = Col::from(usize::from(r));

        Pos::from((r_prime, c_prime))
    }
    /// Finds the sum of the scores of each word. If an invalid word is
    /// encountered, returns an error, otherwise returns the sum of the
    /// scores of all words containing new letters. `occ` is the combined
    /// occupancy of the board and new tiles. `map_pos` is used to rotate
    /// the bit positions back to the standard grid for the rotated vertical
    /// bitboard.
    fn score_words<F>(
        &self,
        occ: BitBoard,
        new: BitBoard,
        word_tree: &WordTree,
        map_pos: F,
    ) -> GameResult<usize>
    where
        F: Copy + Fn(Pos) -> Pos,
    {
        let mut sum = 0;

        let mut bits = new.into_iter();
        let mut curr_bit = bits.next();

        // a word has at most 15 letters.
        let mut letter_multipliers = [1; 15];

        for word in WordBoundaries::new(occ) {
            if let Some(pos) = curr_bit {
                if word.contains(pos) {
                    let mut word_multiplier = 1;
                    letter_multipliers.fill(1);

                    let mut curr_node = word_tree.root_idx();

                    while let Some(pos) = curr_bit {
                        // stop looping once `pos` is no longer within the word
                        if !word.contains(pos) {
                            break;
                        }

                        let real_pos = map_pos(pos);
                        let tile = self.at(real_pos).expect("An occupied square");
                        let letter = tile.letter().expect("A letter");

                        curr_node = word_tree
                            .node(curr_node)
                            .get_child(letter)
                            .ok_or(GameError::InvalidWord)?;
                        curr_bit = bits.next();

                        if let Some(bonus) = real_pos.bonus() {
                            let offset = usize::from(pos - word.start());

                            word_multiplier *= bonus.word_multiplier();
                            letter_multipliers[offset] = bonus.letter_multiplier();
                        }
                    }

                    if !word_tree.node(curr_node).is_terminal() {
                        return Err(GameError::InvalidWord);
                    }

                    sum += word
                        .iter_range()
                        .map(map_pos)
                        .filter_map(|real_pos| self.at(real_pos))
                        .enumerate()
                        .map(|(offset, tile)| tile.score() * letter_multipliers[offset])
                        .sum::<usize>()
                        * word_multiplier;
                }
            }
        }

        Ok(sum)
    }
    /// Calculates the set of tiles with at least one neighbour in any
    /// of the 4 directions. (up,down,left,right).
    fn neighbours(occ: BitBoard) -> BitBoard {
        let mut tiles_with_neighbours = BitBoard::full();

        // finds set of tiles with neighbours in a direction.
        let neighbours = |mask: BitBoard, shift: i32| {
            let masked = occ & !mask;
            let shifted = if shift < 0 {
                masked >> (-shift) as usize
            } else {
                masked << shift as usize
            };
            occ & shifted
        };

        tiles_with_neighbours |= neighbours(BitBoard::bottom_row(), 15);
        tiles_with_neighbours |= neighbours(BitBoard::top_row(), -15);
        tiles_with_neighbours |= neighbours(BitBoard::rightmost_col(), 1);
        tiles_with_neighbours |= neighbours(BitBoard::leftmost_col(), -1);

        tiles_with_neighbours
    }
    /// Validates a position based on the locations of the tiles.
    ///
    /// Ensures that these conditions are met:
    /// * The set of all tiles must always contain the start tile.
    /// * Each tile must be adjacent to another which can be to its
    ///   left, right, above or below.
    /// * Every word has at least 2 letters
    fn validate_occ(occ: BitBoard) -> GameResult<()> {
        // there must be a tile on the start square.
        if !occ.is_bit_set(Pos::start()) {
            return Err(GameError::MustIntersectStart);
        }

        // every word needs at least two letters, hence the bit count
        // (total number of tiles) must be greater than 1.
        if occ.bit_count() < 2 {
            return Err(GameError::WordsNeedTwoLetters);
        }

        // every tile must have a neighbour in one of the 4 directions
        let without_neighbours = !Self::neighbours(occ);
        if !without_neighbours.is_zero() {
            return Err(GameError::NotConnected);
        }

        Ok(())
    }
    /// Gets the tile at `pos`
    pub fn at<T>(&self, pos: T) -> Option<Tile>
    where
        T: Into<Pos>,
    {
        self.grid[usize::from(pos.into())]
    }
    /// Removes all tiles in `tile_positions` from the board.
    pub fn undo_placement(&mut self, tile_positions: Vec<Pos>) {
        for pos in tile_positions {
            self.grid[usize::from(pos)] = None;
            self.occupancy_h.clear_bit(pos);
            self.occupancy_v.clear_bit(Self::anti_clockwise_90(pos));
        }
    }
    /// Attempts to perform a [`Play::Place`] on the board. (All
    /// other variants don't require board modification). If succesful,
    /// returns the score from placing the new tiles.
    pub fn make_placement(
        &mut self,
        tile_positions: Vec<(Pos, Tile)>,
        word_tree: &WordTree,
    ) -> GameResult<usize> {
        if tile_positions.is_empty() {
            return Err(GameError::ZeroTilesPlaced);
        }

        // at most 7 tiles can be placed
        if tile_positions.len() > 7 {
            return Err(GameError::MaximumTilesExceeded);
        }

        // new tiles for horizontal words
        let mut new_tiles_h = BitBoard::default();
        // new tiles for vertical words: rotated 90deg anticlockwise
        let mut new_tiles_v = BitBoard::default();

        for &(pos_h, _) in &tile_positions {
            // if the bit has already been set then `tile_positions` contains
            // a duplicate tile.
            if new_tiles_h.is_bit_set(pos_h) {
                return Err(GameError::DuplicatePosition);
            }

            new_tiles_h.set_bit(pos_h);
            new_tiles_v.set_bit(Self::anti_clockwise_90(pos_h));
        }

        // check whether the new tiles overlay the current tiles.
        if self.occupancy_h.intersects(&new_tiles_h) {
            return Err(GameError::CoincedentTiles);
        }

        // find a combined set of tiles
        let occ_h = self.occupancy_h | new_tiles_h;
        let occ_v = self.occupancy_v | new_tiles_v;

        // perform tile placement validation
        Self::validate_occ(occ_h)?;

        // the placement of tiles has now been validated, now
        // the words formed by the tiles must be validated.
        let score = self.score_words(occ_h, new_tiles_h, word_tree, |pos| pos)?
            + self.score_words(occ_v, new_tiles_v, word_tree, Self::clockwise_90)?;

        // everything has now been validated: place the tiles on the board.
        for (pos, tile) in tile_positions {
            self.grid[usize::from(pos)] = Some(tile);
        }

        Ok(score)
    }
}
impl Default for Board {
    fn default() -> Self {
        Self {
            grid: [None; CELLS],
            occupancy_h: BitBoard::default(),
            occupancy_v: BitBoard::default(),
        }
    }
}
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // print col headers
        write!(f, "   ")?;
        for col in Col::iter() {
            write!(f, " {} ", col)?;
        }
        writeln!(f)?;

        for row in Row::iter() {
            // print row header
            write!(f, "{:>2} ", row.to_string())?;

            for col in Col::iter() {
                match self.at((row, col)) {
                    Some(tile) => write!(f, "{}", tile)?,
                    None => write!(f, " . ")?,
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}
