//! Structures for iterating over the words in a bitboard.

use super::{
    bitboard::{BitBoard, Bits},
    pos::Pos,
};

/// Used to iterate over complete words in a bitboard.
#[derive(Debug)]
pub struct Words {
    word_boundaries: Bits,
    rotate_positions: bool,
}
impl Words {
    /// Creates a new `Words` iterator from the **normal occupancy**.
    pub fn horizontal(occ: BitBoard) -> Self {
        Self {
            word_boundaries: Bits::from(occ),
            rotate_positions: false,
        }
    }
    /// Creates a new `Words` iterator for vertical words from
    /// the **rotated occupancy**.
    pub fn vertical(occ: BitBoard) -> Self {
        Self {
            word_boundaries: Bits::from(occ),
            rotate_positions: true,
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

        Some(match self.rotate_positions {
            true => Word {
                curr: curr.clockwise90(),
                end: end.clockwise90(),
            },
            false => Word { curr, end },
        })
    }
}

/// Used to iterate over the positions ([`Pos`]) in a word. Positons from
/// this iterator have been mapped back to the horizontal versions.
#[derive(Debug)]
pub struct Word {
    curr: Pos,
    end: Pos,
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
                self.curr = self.curr.next();
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
