//! Models the scrabble board.

use crate::{
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
        Self(occ.word_boundaries().into_iter())
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
    occ_h: BitBoard,
    /// vertical occupancy, rotated 90deg. For finding vertical words.
    occ_v: BitBoard,
}
impl Board {
    /// Finds the sum of the scores of each word. If an invalid word is
    /// encountered, returns an error, otherwise returns the sum of the
    /// scores of all words containing new letters. `map_pos` is used to rotate
    /// the bit positions back to the standard grid for the rotated vertical
    /// bitboard. `new` is the set of added tiles, which should ave been
    /// rotated previously for vertical words. `occ` is the set of existing
    /// tiles, which should also have been rotated.
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

        // find the combined occupancy
        let occ = occ | new;

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
    /// Computes the combined score for horizontal and vertical words, adding
    /// the 50 point bonus where appropriate.
    fn score_and_validate(
        &self,
        new_h: BitBoard,
        new_v: BitBoard,
        word_tree: &WordTree,
    ) -> GameResult<usize> {
        // Find the score for horizontal and vertical words.
        let score_h = self.score_words(self.occ_h, new_h, word_tree, |pos| pos)?;
        let score_v = self.score_words(self.occ_v, new_v, word_tree, |pos| pos.clockwise90())?;

        // Find combined score
        let score = score_h + score_v;

        // If the bitcount for `new_h` is 7, add a 50 point bonus
        match new_h.bit_count() {
            7 => Ok(score + 50),
            _ => Ok(score),
        }
    }
    /// Validates a position based on the locations of the tiles.
    /// Bitboards provided should be horizontal, ie. the natural
    /// layout of the board.
    ///
    /// Ensures that these conditions are met:
    /// * The new tiles cannot intersect the old tiles.
    /// * The set of all tiles must always contain the start tile.
    /// * There must be a path from the start tile to any other tile.
    /// * Every word has at least 2 letters.
    fn validate_occ_h(occ_h: BitBoard, mut new_h: BitBoard) -> GameResult<()> {
        // Check whether the new tiles intersect the old tiles
        if occ_h.intersects(&new_h) {
            return Err(GameError::CoincedentTiles);
        }

        // Find the combined occupancy
        let occ = occ_h | new_h;

        // there must be a tile on the start square.
        if !occ.is_bit_set(Pos::start()) {
            return Err(GameError::MustIntersectStart);
        }

        // every word needs at least two letters, hence the bit count
        // (total number of tiles) must be greater than 1.
        if occ.bit_count() < 2 {
            return Err(GameError::WordsNeedTwoLetters);
        }

        // Every tile must be connected. However, it can be assumed
        // that the existing tiles (`occ_h`) are already connected,
        // so consider the neighbouring tiles in `new_h`. Since there are at
        // most 7 new tiles, this loop will run at most 7 times.

        // Start with the current occupancy (assume that `occ_h` is connected).
        let mut connected = occ_h;

        // Set the start bit, required for first move when occupancy
        // is zero.
        connected.set_bit(Pos::start());

        // remove the start bit from `new_h`
        new_h.clear_bit(Pos::start());

        // Keep looping until there are no neighbours
        loop {
            // Find the set of new tiles which neighbours the connected
            // set of tiles.
            let neighbours = connected.neighbours() & new_h;

            // Remove the tiles from the set of tiles to consider
            new_h ^= neighbours;
            // Add the tiles to the set of connected tiles.
            connected |= neighbours;

            // if there are no neighbouring tiles, then exit the loop
            if neighbours.is_zero() {
                // exits the loop and returns a value
                break match new_h.is_zero() {
                    // if there are still tiles remaining in `new_h` then
                    // the tiles are not connected.
                    false => Err(GameError::NotConnected),
                    // otherwise all tiles are connected.
                    true => Ok(()),
                };
            }
        }
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
            self.occ_h.clear_bit(pos);
            self.occ_v.clear_bit(pos.anti_clockwise90());
        }
    }
    /// Attempts to perform a [`Play::Place`] on the board. (All
    /// other variants don't require board modification). If succesful,
    /// returns the score from placing the new tiles.
    pub fn make_placement(
        &mut self,
        tile_positions: &[(Pos, Tile)],
        word_tree: &WordTree,
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
        Self::validate_occ_h(self.occ_h, new_h)?;

        // Tiles positions have now been validated: place the tiles on the board.
        // Word validation requires that these tiles are present. If an invalid
        // word exists on the board, the tiles will be removed.
        for &(pos, tile) in tile_positions {
            self.grid[usize::from(pos)] = Some(tile);
        }

        // checks that words are valid then returns the score
        match self.score_and_validate(new_h, new_v, word_tree) {
            // everything was valid, update the bitboards.
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
        write_grid(f, |pos| match self.at(pos) {
            Some(tile) => format!("{}", tile),
            None => " . ".to_string(),
        })
    }
}

/// Utility function for displaying a grid, which prints row
/// and column headers. `at_pos` should return a string of length
/// 3 which represents the cell at the provided position.
///
/// This function is used for implementing [`fmt::Display`] for [`Board`]
/// and [`BitBoard`].
pub fn write_grid<F, T>(f: &mut fmt::Formatter, at_pos: F) -> fmt::Result
where
    F: Fn(Pos) -> T,
    T: fmt::Display,
{
    fn write_col_headers(f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "   ")?;
        for col in Col::iter() {
            write!(f, " {} ", col)?;
        }

        Ok(())
    }

    write_col_headers(f)?;

    writeln!(f)?;

    for row in Row::iter() {
        write!(f, "{:>2} ", row.to_string())?;

        for col in Col::iter() {
            write!(f, "{}", at_pos(Pos::from((row, col))))?;
        }

        writeln!(f, " {:<2}", row.to_string())?;
    }

    write_col_headers(f)
}
