//! Structures for iterating over the words in a bitboard.

use crate::{
    game::board::COLS,
    util::{
        bitboard::{BitBoard, Bits},
        pos::{Direction, Pos},
    },
};
use std::{iter::StepBy, ops::RangeInclusive};

/// A start and end position for a word.
#[derive(Debug)]
pub struct Boundary {
    start: Pos,
    end: Pos,
}
impl Boundary {
    /// Tests whether a position is within the horizontal
    /// span of the [`WordBoundary`].
    pub fn contains(&self, pos: Pos) -> bool {
        self.start <= pos && pos <= self.end
    }
}

/// Iterates over the (start, end) position tuples on the board.
#[derive(Debug)]
pub struct Boundaries {
    word_boundaries: Bits,
}
impl From<BitBoard> for Boundaries {
    fn from(occ: BitBoard) -> Self {
        Self {
            word_boundaries: Bits::from(occ.words_h()),
        }
    }
}
impl Iterator for Boundaries {
    type Item = Boundary;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.word_boundaries.next()?;
        let end = self.word_boundaries.next()?;

        Some(Boundary { start, end })
    }
}

/// Adapts [`WordBoundaries`] to iterate over
#[derive(Debug)]
pub struct Intersecting<I> {
    word_boundaries: I,
    new: Bits,
    curr: Option<Pos>,
}
impl<I: Iterator<Item = Boundary>> Iterator for Intersecting<I> {
    type Item = Boundary;

    fn next(&mut self) -> Option<Self::Item> {
        // if the current pos is `None` then stop.
        let curr = self.curr?;

        loop {
            // advance the `words` iterator until the current position
            // is within the current word. (skip all words which do not
            // contain new tiles).
            let wb = self.word_boundaries.next()?;
            if wb.contains(curr) {
                // this word will be returned as the next item, but the current
                // position needs to be advanced until it is no longer within the
                // word.
                loop {
                    self.curr = self.new.next();

                    match self.curr {
                        // no more bits so return the word.
                        None => break,
                        // check that the current bit is no longer within the word.
                        Some(curr) if !wb.contains(curr) => break,
                        // otherwise continue to advance the bit iterator.
                        _ => continue,
                    }
                }

                return Some(wb);
            }
        }
    }
}

/// Adapts an iterator over [`WordBoundary`]s to an iterator
/// over horizontal [`Word`]s.
#[derive(Debug)]
pub struct Horizontal<I> {
    inner: I,
}
impl<I: Iterator<Item = Boundary>> Iterator for Horizontal<I> {
    type Item = Word;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|Boundary { start, end }| Word {
            start,
            end,
            dir: Direction::East,
        })
    }
}

/// Adapts an iterator over [`WordBoundary`]s to an iterator
/// over vertical [`Word`]s.
#[derive(Debug)]
pub struct Vertical<I> {
    inner: I,
}
impl<I: Iterator<Item = Boundary>> Iterator for Vertical<I> {
    type Item = Word;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|Boundary { start, end }| Word {
            start: start.clockwise90(),
            end: end.clockwise90(),
            dir: Direction::South,
        })
    }
}

/// Used to iterate over the positions ([`Pos`]) in a word. Positons from
/// this iterator have been mapped back to the horizontal versions.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Word {
    start: Pos,
    end: Pos,
    dir: Direction,
}
impl IntoIterator for Word {
    type IntoIter = WordIntoIter;
    type Item = Pos;

    fn into_iter(self) -> Self::IntoIter {
        let start = usize::from(self.start);
        let end = usize::from(self.end);
        let inc = match self.dir {
            Direction::East => 1,
            Direction::South => COLS,
            // words can only go left to right or down.
            _ => unreachable!(),
        };

        WordIntoIter {
            range: (start..=end).step_by(inc),
        }
    }
}

/// Struct used to iterate over the positions in a word.
pub struct WordIntoIter {
    range: StepBy<RangeInclusive<usize>>,
}
impl Iterator for WordIntoIter {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        self.range.next().map(Pos::from)
    }
}

/// Iterator extension trait that makes the API for this
/// module more ergonomic.
pub trait WordsIteratorExt: Sized {
    /// Gets an iterator over horizontal words.
    fn horizontal(self) -> Horizontal<Self>;
    /// Gets an iterator over vertical words.
    fn vertical(self) -> Vertical<Self>;
    /// Creates an iterator over only new words (words for which
    /// at least one letter is in the `new` bitboard).
    fn intersecting(self, new: BitBoard) -> Intersecting<Self>;
}
impl<I: Iterator<Item = Boundary>> WordsIteratorExt for I {
    fn horizontal(self) -> Horizontal<Self> {
        Horizontal { inner: self }
    }
    fn vertical(self) -> Vertical<Self> {
        Vertical { inner: self }
    }
    fn intersecting(self, new: BitBoard) -> Intersecting<Self> {
        let mut bits = Bits::from(new);
        let curr = bits.next();

        Intersecting {
            word_boundaries: self,
            new: bits,
            curr,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter;

    fn place_word(bb: &mut BitBoard, start: Pos, dir: Direction, len: usize) {
        for pos in iter::successors(Some(start), |x| x.offset(dir, 1)).take(len) {
            bb.set(pos);
        }
    }

    #[test]
    fn horizontal() {
        let mut bb = BitBoard::default();
        place_word(&mut bb, Pos::start(), Direction::East, 4);

        let mut words = bb.word_boundaries().horizontal();

        let first = words.next().expect("at least one word");

        assert_eq!(
            first,
            Word {
                start: Pos::start(),
                end: Pos::start().offset(Direction::East, 3).unwrap(),
                dir: Direction::East,
            }
        );
        assert!(words.next().is_none());
    }

    #[test]
    fn vertical() {
        let mut bb = BitBoard::default();
        place_word(&mut bb, Pos::start(), Direction::East, 7);

        let mut words = bb.word_boundaries().vertical();

        let first = words.next().expect("at least one word");

        assert_eq!(
            first,
            Word {
                start: Pos::start(),
                end: Pos::start().offset(Direction::South, 6).unwrap(),
                dir: Direction::South,
            }
        );
        assert!(words.next().is_none());
    }

    #[test]
    fn empty() {
        assert!(BitBoard::default()
            .word_boundaries()
            .horizontal()
            .next()
            .is_none());
        assert!(BitBoard::default()
            .word_boundaries()
            .vertical()
            .next()
            .is_none());

        let mut horizontal_bb = BitBoard::default();
        place_word(&mut horizontal_bb, Pos::start(), Direction::North, 7);
        assert!(horizontal_bb
            .word_boundaries()
            .horizontal()
            .next()
            .is_none());

        let mut vertical_bb = BitBoard::default();
        place_word(&mut vertical_bb, Pos::start(), Direction::North, 7);
        assert!(vertical_bb.word_boundaries().vertical().next().is_none());
    }

    #[test]
    fn h_new() {
        // create a bitboard with 2 horizontal words, one of which has
        // a letter contained in the `new` bitboard.
        let mut bb = BitBoard::default();
        place_word(&mut bb, Pos::start(), Direction::East, 4);
        place_word(
            &mut bb,
            Pos::start().offset(Direction::South, 2).unwrap(),
            Direction::East,
            6,
        );

        let mut new = BitBoard::default();
        new.set(
            Pos::start()
                .offset(Direction::South, 2)
                .unwrap()
                .offset(Direction::East, 2)
                .unwrap(),
        );

        // two words in total
        let mut words = bb.word_boundaries().horizontal();
        assert!(words.next().is_some());
        assert!(words.next().is_some());
        assert!(words.next().is_none());

        // only a single word with a new letter
        let mut new_words = bb.word_boundaries().intersecting(new).horizontal();
        assert_eq!(
            new_words.next(),
            Some(Word {
                start: Pos::start().offset(Direction::South, 2).unwrap(),
                end: Pos::start()
                    .offset(Direction::South, 2)
                    .unwrap()
                    .offset(Direction::East, 5)
                    .unwrap(),
                dir: Direction::East,
            })
        );
        assert!(new_words.next().is_none());
    }
}
