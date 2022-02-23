//! Structure for iterating over the word boundaries (start, end) pairs
//! in a bitboard.

use super::{
    bitboard::{BitBoard, Bits},
    pos::Pos,
};

/// Struct wrapping the result of `BitBoard::word_boundaries` which
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
