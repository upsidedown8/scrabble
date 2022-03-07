//! Structures for iterating over the words in a bitboard.

use super::{
    bitboard::{BitBoard, Bits},
    pos::{Direction, Pos},
};

/// Used to iterate over complete words in a bitboard.
#[derive(Debug)]
pub struct Words {
    word_boundaries: Bits,
    dir: Direction,
}
impl Words {
    /// Creates a new `Words` iterator from the **normal occupancy**.
    pub fn horizontal(occ_h: BitBoard) -> Self {
        Self {
            word_boundaries: Bits::from(occ_h.words_h()),
            dir: Direction::East,
        }
    }
    /// Creates a new `Words` iterator for vertical words from
    /// the **rotated occupancy**.
    pub fn vertical(occ_v: BitBoard) -> Self {
        Self {
            word_boundaries: Bits::from(occ_v.words_h()),
            dir: Direction::South,
        }
    }
    /// Gets an iterator that only contains newly placed words. `new`
    /// is a (horizontal) bitboard containing the newly placed tiles.
    pub fn new_words(self, new: BitBoard) -> NewWords {
        let mut new = Bits::from(new);
        let curr = new.next();

        NewWords {
            words: self,
            new,
            curr,
        }
    }
}
impl Iterator for Words {
    type Item = Word;

    fn next(&mut self) -> Option<Word> {
        let curr = self.word_boundaries.next()?;
        let end = self.word_boundaries.next()?;

        Some(match self.dir {
            Direction::South => Word {
                curr: curr.clockwise90(),
                end: end.clockwise90(),
                dir: self.dir,
            },
            _ => Word {
                curr,
                end,
                dir: self.dir,
            },
        })
    }
}

/// Used to iterate over the positions ([`Pos`]) in a word. Positons from
/// this iterator have been mapped back to the horizontal versions.
#[derive(Debug, PartialEq, Eq)]
pub struct Word {
    curr: Pos,
    end: Pos,
    dir: Direction,
}
impl Word {
    /// Checks whether a position is contained within the word.
    pub fn contains(&self, pos: Pos) -> bool {
        self.curr <= pos && pos <= self.end
    }
}
impl Iterator for Word {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        match self.curr > self.end {
            true => None,
            false => {
                let pos = self.curr;
                self.curr = self.curr.offset(self.dir, 1).unwrap();
                Some(pos)
            }
        }
    }
}

/// Used to iterate over words which contain at least one new tile.
#[derive(Debug)]
pub struct NewWords {
    words: Words,
    new: Bits,
    curr: Option<Pos>,
}
impl Iterator for NewWords {
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

        let mut words = Words::horizontal(bb);

        let first = words.next().expect("at least one word");

        assert_eq!(
            first,
            Word {
                curr: Pos::start(),
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

        let mut words = Words::vertical(bb);

        let first = words.next().expect("at least one word");

        assert_eq!(
            first,
            Word {
                curr: Pos::start(),
                end: Pos::start().offset(Direction::South, 6).unwrap(),
                dir: Direction::South,
            }
        );
        assert!(words.next().is_none());
    }

    #[test]
    fn empty() {
        assert!(Words::horizontal(BitBoard::default()).next().is_none());
        assert!(Words::vertical(BitBoard::default()).next().is_none());

        let mut horizontal_bb = BitBoard::default();
        place_word(&mut horizontal_bb, Pos::start(), Direction::North, 7);
        assert!(Words::horizontal(horizontal_bb).next().is_none());

        let mut vertical_bb = BitBoard::default();
        place_word(&mut vertical_bb, Pos::start(), Direction::North, 7);
        assert!(Words::vertical(vertical_bb).next().is_none());
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
        let mut words = Words::horizontal(bb);
        assert!(words.next().is_some());
        assert!(words.next().is_some());
        assert!(words.next().is_none());

        // only a single word with a new letter
        let mut new_words = Words::horizontal(bb).new_words(new);
        assert_eq!(
            new_words.next(),
            Some(Word {
                curr: Pos::start().offset(Direction::South, 2).unwrap(),
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
