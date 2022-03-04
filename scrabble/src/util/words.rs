//! Structures for iterating over the words in a bitboard.

use super::{
    bitboard::{BitBoard, Bits},
    pos::Pos,
};

/// Used to iterate over complete words in a bitboard.
#[derive(Clone, Copy, Debug)]
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
#[derive(Clone, Debug)]
pub struct Word {
    curr: Pos,
    end: Pos,
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
