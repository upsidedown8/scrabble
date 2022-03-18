//! Structures for iterating over the words in a bitboard.

use super::{
    bitboard::{BitBoard, Bits, ReverseBits},
    pos::{Direction, Pos},
};

/// Used to iterate over complete horizontal words in a bitboard.
#[derive(Debug)]
pub struct HorizontalWords {
    word_boundaries: Bits,
}
impl HorizontalWords {
    /// Creates a new `HorizontalWords` iterator from the **normal occupancy**.
    pub fn new(occ_h: BitBoard) -> Self {
        Self {
            word_boundaries: Bits::from(occ_h.words_h()),
        }
    }
    /// Creates an iterator over only the new horizontal words.
    pub fn new_words(self, new_h: BitBoard) -> NewWords<Self> {
        let mut new = Bits::from(new_h);
        let curr = new.next();

        NewWords {
            words: self,
            new,
            curr,
        }
    }
}
impl Iterator for HorizontalWords {
    type Item = Word;

    fn next(&mut self) -> Option<Word> {
        let start = self.word_boundaries.next()?;
        let end = self.word_boundaries.next()?;
        Some(Word {
            start,
            end,
            dir: Direction::East,
        })
    }
}

/// Used to iterate over complete vertical words in a bitboard.
#[derive(Debug)]
pub struct VerticalWords {
    word_boundaries: ReverseBits,
}
impl VerticalWords {
    /// Creates a new `Words` iterator for vertical words from
    /// the **rotated occupancy**.
    pub fn new(occ_v: BitBoard) -> Self {
        Self {
            word_boundaries: ReverseBits::from(occ_v.words_h()),
        }
    }
    /// Creates an iterator over only the new vertical words.
    pub fn new_words(self, new_h: BitBoard) -> NewWords<Self> {
        let mut new = Bits::from(new_h);
        let curr = new.next();

        NewWords {
            words: self,
            new,
            curr,
        }
    }
}
impl Iterator for VerticalWords {
    type Item = Word;

    fn next(&mut self) -> Option<Word> {
        // since iteration is in reverse, start & end are flipped
        let end = self.word_boundaries.next()?;
        let start = self.word_boundaries.next()?;
        Some(Word {
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
impl Word {
    /// Checks whether a position is contained within the word.
    pub fn contains(&self, pos: Pos) -> bool {
        // the position should be numerically between the start and end,
        // and in the same row or col, depending on `self.dir`.
        self.start <= pos && pos <= self.end && match self.dir {
            Direction::South => pos.col() == self.start.col(),
            // east
            _ => pos.row() == self.start.row(),
        }
    }
    /// Gets the direction of the word.
    pub fn dir(&self) -> Direction {
        self.dir
    }
}
impl IntoIterator for Word {
    type IntoIter = WordIntoIter;
    type Item = Pos;

    fn into_iter(self) -> Self::IntoIter {
        WordIntoIter {
            curr: Some(self.start),
            end: self.end,
            dir: self.dir,
        }
    }
}

/// Struct used to iterate over the positions in a word.
pub struct WordIntoIter {
    curr: Option<Pos>,
    end: Pos,
    dir: Direction
}
impl Iterator for WordIntoIter {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.curr?;
        self.curr = match curr.dir(self.dir) {
            Some(next_pos) if next_pos <= self.end => Some(next_pos),
            _ => None,
        };
        Some(curr)
    }
}

/// Used to iterate over words which contain at least one new tile.
#[derive(Debug)]
pub struct NewWords<I> {
    words: I,
    new: Bits,
    curr: Option<Pos>,
}
impl<I: Iterator<Item = Word>> Iterator for NewWords<I> {
    type Item = Word;

    fn next(&mut self) -> Option<Self::Item> {
        // if the current pos is `None` then stop.
        let curr = self.curr?;

        loop {
            // advance the `words` iterator until the current position
            // is within the current word. (skip all words which do not
            // contain new tiles).
            let word = self.words.next()?;
            if word.contains(curr) {
                // this word will be returned as the next item, but the current
                // position needs to be advanced until it is no longer within the
                // word.
                loop {
                    self.curr = self.new.next();

                    match self.curr {
                        // no more bits so return the word.
                        None => break,
                        // check that the current bit is no longer within the word.
                        Some(curr) if !word.contains(curr) => break,
                        // otherwise continue to advance the bit iterator.
                        _ => continue,
                    }
                }

                return Some(word);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter;

    fn place_word(bb: &mut BitBoard, start: Pos, dir: Direction, len: usize) {
        for pos in iter::successors(Some(start), |x| x.offset(dir, 1)).take(len) {
            bb.set_bit(pos);
        }
    }

    #[test]
    fn horizontal() {
        let mut bb = BitBoard::default();
        place_word(&mut bb, Pos::start(), Direction::East, 4);

        let mut words = HorizontalWords::new(bb);

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

        let mut words = VerticalWords::new(bb);

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
        assert!(HorizontalWords::new(BitBoard::default()).next().is_none());
        assert!(VerticalWords::new(BitBoard::default()).next().is_none());

        let mut horizontal_bb = BitBoard::default();
        place_word(&mut horizontal_bb, Pos::start(), Direction::North, 7);
        assert!(HorizontalWords::new(horizontal_bb).next().is_none());

        let mut vertical_bb = BitBoard::default();
        place_word(&mut vertical_bb, Pos::start(), Direction::North, 7);
        assert!(VerticalWords::new(vertical_bb).next().is_none());
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
        new.set_bit(
            Pos::start()
                .offset(Direction::South, 2)
                .unwrap()
                .offset(Direction::East, 2)
                .unwrap(),
        );

        // two words in total
        let mut words = HorizontalWords::new(bb);
        assert!(words.next().is_some());
        assert!(words.next().is_some());
        assert!(words.next().is_none());

        // only a single word with a new letter
        let mut new_words = HorizontalWords::new(bb).new_words(new);
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
