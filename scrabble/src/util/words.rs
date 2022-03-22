//! Structures for iterating over the words in a bitboard.

use std::fmt;

use crate::{
    game::tile::Tile,
    util::{
        bitboard::{BitBoard, Bits},
        grid::Grid,
        pos::Pos,
    },
};

/// Stores a start and end position for a word.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct WordBoundary {
    start: Pos,
    end: Pos,
}
impl WordBoundary {
    /// Tests whether a position is within the range
    /// `start..=end`.
    pub fn contains(&self, pos: Pos) -> bool {
        self.start <= pos && pos <= self.end
    }
}
impl IntoIterator for WordBoundary {
    type Item = Pos;
    type IntoIter = WordBoundaryIter;

    fn into_iter(self) -> Self::IntoIter {
        WordBoundaryIter {
            curr: self.start,
            end: self.end,
            complete: false,
        }
    }
}

/// Used to iterate over the positions in a [`WordBoundary`].
#[derive(Debug)]
pub struct WordBoundaryIter {
    curr: Pos,
    end: Pos,
    complete: bool,
}
impl Iterator for WordBoundaryIter {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        match self.complete {
            true => None,
            false => {
                let item = self.curr;

                self.complete = self.curr >= self.end;
                self.curr = self.curr.next();

                Some(item)
            }
        }
    }
}

/// Iterates over the (start, end) position tuples for words
/// on a bitboard.
#[derive(Debug)]
pub struct WordBoundaries {
    word_boundaries: Bits,
}
impl WordBoundaries {
    /// Creates a new [`WordBoundaries`] iterator from a bitboard.
    pub fn new(occ: BitBoard) -> Self {
        WordBoundaries {
            word_boundaries: Bits::from(occ.words_h()),
        }
    }
}
impl Iterator for WordBoundaries {
    type Item = WordBoundary;

    fn next(&mut self) -> Option<Self::Item> {
        Some(WordBoundary {
            start: self.word_boundaries.next()?,
            end: self.word_boundaries.next()?,
        })
    }
}

/// Adapts an iterator over [`WordBoundary`]s to only yield those
/// that intersect the provided bitboard.
#[derive(Debug)]
pub struct Intersecting<I> {
    word_boundaries: I,
    new: Bits,
    curr: Option<Pos>,
}
impl<I: Iterator<Item = WordBoundary>> Iterator for Intersecting<I> {
    type Item = WordBoundary;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let boundary = self.word_boundaries.next()?;

            // skip new positions that come before
            while self.curr? < boundary.start {
                self.curr = self.new.next();
            }

            // check whether current position is within boundary
            if boundary.contains(self.curr?) {
                return Some(boundary);
            }
        }
    }
}

/// Adapts an iterator over [`WordBoundary`]s to an iterator over
/// the [`Word`]s on a [`Board`](crate::game::board::Board).
#[derive(Debug)]
pub struct Words<'a, I> {
    word_boundaries: I,
    grid: &'a Grid,
}
impl<'a, I: Iterator<Item = WordBoundary>> Iterator for Words<'a, I> {
    type Item = Word<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Word {
            grid: self.grid,
            boundary: self.word_boundaries.next()?.into_iter(),
        })
    }
}

/// An iterator over the ([`Pos`], [`Tile`]) tuples in a word.
pub struct Word<'a> {
    grid: &'a Grid,
    boundary: WordBoundaryIter,
}
impl<'a> Iterator for Word<'a> {
    type Item = (Pos, Tile);

    fn next(&mut self) -> Option<Self::Item> {
        let pos: Pos = self.boundary.next()?;
        Some((pos, self.grid[pos]?))
    }
}
impl<'a> fmt::Debug for Word<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Word")
            .field("boundary", &self.boundary)
            .finish()
    }
}
impl<'a> fmt::Display for Word<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let WordBoundaryIter { curr, end, .. } = &self.boundary;
        let curr = usize::from(*curr);
        let end = usize::from(*end);
        for pos in (curr..=end).map(Pos::from) {
            write!(
                f,
                "{}",
                self.grid[pos].expect("a tile").letter().expect("a letter")
            )?;
        }

        Ok(())
    }
}

/// Provides implementations of the `words` and `intersecting` methods
/// for any iterator over [`WordBoundary`]s.
pub trait WordsExt: Sized {
    /// Gets an iterator over the [`Word`]s referred to by each
    /// [`WordBoundary`].
    fn words(self, grid: &'_ Grid) -> Words<'_, Self>;
    /// Gets the word boundaries that intersect the `new` bitboard.
    fn intersecting(self, occ: BitBoard) -> Intersecting<Self>;
}
impl<I: Iterator<Item = WordBoundary>> WordsExt for I {
    fn words(self, grid: &'_ Grid) -> Words<'_, I> {
        Words {
            word_boundaries: self,
            grid,
        }
    }
    fn intersecting(self, occ: BitBoard) -> Intersecting<Self> {
        let mut new = Bits::from(occ);
        let curr = new.next();

        Intersecting {
            word_boundaries: self,
            new,
            curr,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::pos::Direction;

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

        let mut words = bb.word_boundaries();

        let first = words.next().expect("at least one word");

        assert_eq!(
            first,
            WordBoundary {
                start: Pos::start(),
                end: Pos::start().offset(Direction::East, 3).unwrap(),
            }
        );
        assert!(words.next().is_none());
    }

    #[test]
    fn empty() {
        assert!(BitBoard::default().word_boundaries().next().is_none());

        let mut horizontal_bb = BitBoard::default();
        place_word(&mut horizontal_bb, Pos::start(), Direction::North, 7);
        assert!(horizontal_bb.word_boundaries().next().is_none());
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
        let mut words = bb.word_boundaries();
        assert!(words.next().is_some());
        assert!(words.next().is_some());
        assert!(words.next().is_none());

        // only a single word with a new letter
        let mut new_words = bb.word_boundaries().intersecting(new);
        assert_eq!(
            new_words.next(),
            Some(WordBoundary {
                start: Pos::start().offset(Direction::South, 2).unwrap(),
                end: Pos::start()
                    .offset(Direction::South, 2)
                    .unwrap()
                    .offset(Direction::East, 5)
                    .unwrap(),
            })
        );
        assert!(new_words.next().is_none());
    }
}
