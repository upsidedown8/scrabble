//! Module containing functions and structs which are used across
//! the library.

use crate::{
    error::{GameError, GameResult},
    util::{
        bitboard::BitBoard,
        pos::{Col, Pos, Row},
    },
};
use std::fmt;

pub mod bitboard;
pub mod fsm;
pub mod grid;
pub mod pos;
pub mod scoring;
pub mod tile_counts;
pub mod words;

/// Utility function for displaying a grid, which prints row
/// and column headers. `at_pos` should return a string of length
/// 3 which represents the cell at the provided position.
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

/// Validates a position based on the locations of the tiles.
/// Bitboards provided should be horizontal, ie. the natural
/// layout of the board.
///
/// Ensures that these conditions are met:
/// * The new tiles cannot intersect the old tiles.
/// * The set of all tiles must always contain the start tile.
/// * There must be a path from the start tile to any other tile.
/// * Every word has at least 2 letters.
pub fn validate_occ_h(occ_h: BitBoard, mut new_h: BitBoard) -> GameResult<()> {
    // Check whether the new tiles intersect the old tiles
    if occ_h.intersects(&new_h) {
        return Err(GameError::CoincedentTiles);
    }

    // Find the combined occupancy
    let occ = occ_h | new_h;

    // there must be a tile on the start square.
    if !occ.is_set(Pos::start()) {
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
    connected.set(Pos::start());

    // remove the start bit from `new_h`
    new_h.clear(Pos::start());

    match is_connected(connected, new_h) {
        // if there are still tiles remaining in `new_h` then
        // the tiles are not connected.
        false => Err(GameError::NotConnected),
        // otherwise all tiles are connected.
        true => Ok(()),
    }
}

/// Checks whether the `new` tiles are connected orthagonally to the already
/// `connected` tiles. The `connected` bitboard is assumed to contain tiles
/// that are already connected together. The `new` bitboard must not intersect
/// the `connected` bitboard.
pub fn is_connected(mut connected: BitBoard, mut new: BitBoard) -> bool {
    // Keep looping until there are no neighbours
    loop {
        // Find the set of new tiles which neighbours the connected
        // set of tiles.
        let neighbours = connected.neighbours() & new;

        // Remove the tiles from the set of tiles to consider
        new ^= neighbours;
        // Add the tiles to the set of connected tiles.
        connected |= neighbours;

        // if there are no neighbouring tiles, then exit the loop
        if neighbours.is_zero() {
            // exits the loop and returns a value
            return new.is_zero();
        }
    }
}

/// Gets a bitboard containing the set of squares on which a
/// horizontal word could start.
pub fn possible_starts_h(occ_h: BitBoard, rack_len: usize) -> BitBoard {
    // find all word stems. that is: all tiles shifted up, down and left.
    let mut stems = occ_h.north() | occ_h.south() | occ_h.west();
    // the start position is also a valid stem.
    stems.set(Pos::start());
    // shift and add `stems` to the left (rack_len - 1) times, as one shift
    // was already performed above.
    for _ in 0..rack_len - 1 {
        stems |= stems.west();
    }
    // exclude any overlap with the existing occupancy.
    let stems = stems & !occ_h;

    // find the starts of all existing words
    let starts = occ_h.word_starts_h();

    // final set of starting positions is the `stems` OR the `starts`,
    // without any squares that have a neighbour to the left (as these
    // squares cannot start a word), minus the rightmost column as no word
    // can start there.
    (stems | starts) & !occ_h.east() & !BitBoard::RIGHT_COL
}
