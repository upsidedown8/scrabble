//! Module containing a bitboard implementation to represent
//! the occupancy on the 15 by 15 board.

use super::{pos::Pos, words::WordBoundaries, write_grid};
use std::{
    fmt,
    ops::{
        BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
        ShrAssign,
    },
};

/// [`WORD_SIZE`] = the number of bits in each word. A [`u32`] could have been
/// used giving [`WORD_SIZE`] = 32.
const WORD_SIZE: usize = 64;
/// Since the bitboard has 64 * 4 = 256 bits, but only 225 are used, there
/// are 256 - 225 = 31 bits extra. These extra bits can sometimes be filled
/// with leftover data from bitwise operations. To ensure that this does not
/// affect the results, after each operation any leftover data is erased by
/// an AND operation with this mask on the final 64 bit word. This mask is the
/// NOT of ((2 << 33) - 1).
const FINAL_WORD_MASK: u64 = 0x1ffffffff;

/// Used to iterate over the bits in a [`BitBoard`].
#[derive(Clone, Copy, Debug)]
pub struct Bits {
    boards: [u64; 4],
    word_idx: usize,
}
impl Iterator for Bits {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        while self.word_idx < 4 {
            let word = self.boards[self.word_idx];

            if word == 0 {
                self.word_idx += 1;
            } else {
                let trailing_zeros = word.trailing_zeros() as usize;
                self.boards[self.word_idx] &= !(1 << trailing_zeros);

                return Some(Pos::from(trailing_zeros + WORD_SIZE * self.word_idx));
            }
        }

        None
    }
}
impl From<BitBoard> for Bits {
    fn from(bb: BitBoard) -> Bits {
        Bits {
            boards: bb.boards,
            word_idx: 0,
        }
    }
}

/// Used to iterate over the bits in a [`BitBoard`] in reverse order.
#[derive(Debug)]
pub struct ReverseBits {
    boards: [u64; 4],
    word_idx: usize,
}
impl Iterator for ReverseBits {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        while self.word_idx < 4 {
            let word = self.boards[3 - self.word_idx];

            if word == 0 {
                self.word_idx += 1;
            } else {
                let leading_zeros = word.leading_zeros() as usize;
                self.boards[3 - self.word_idx] &= !(1 << (63 - leading_zeros));

                return Some(Pos::from(
                    (63 - leading_zeros) + WORD_SIZE * (3 - self.word_idx),
                ));
            }
        }

        None
    }
}
impl From<BitBoard> for ReverseBits {
    fn from(bb: BitBoard) -> ReverseBits {
        ReverseBits {
            boards: bb.boards,
            word_idx: 0,
        }
    }
}

/// Macro that creates a bitboard.
macro_rules! bitboard {
    ($b:expr) => {
        bitboard!($b, $b, $b, $b)
    };
    ($b0:expr, $b1:expr, $b2:expr, $b3:expr) => {
        BitBoard {
            boards: [$b0, $b1, $b2, ($b3) & FINAL_WORD_MASK],
        }
    };
}

/// A scrabble board has [`ROWS`](crate::game::board::ROWS) *
/// [`COLS`](crate::game::board::COLS) = 15 * 15 = 225 squares.
/// The nearest multiple of 64 bit integers is 4, giving 256
/// bit values.
///
/// Using integer types allows for very efficient move generation,
/// validation and scoring, since these operations can be run with a
/// single cpu instruction.
#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub struct BitBoard {
    /// The word type used to meet the required size of a bitboard. Any unsigned type could
    /// be used, but a [`u64`] uses the fewest words so is most efficient.
    boards: [u64; 4],
}
impl BitBoard {
    /// An empty bitboard.
    pub const ZERO: BitBoard = bitboard![0, 0, 0, 0];
    /// A full bitboard.
    pub const FULL: BitBoard = bitboard![u64::MAX];
    /// Only the top row.
    pub const TOP_ROW: BitBoard = bitboard![0x7fff, 0, 0, 0];
    /// Only the bottom row.
    pub const BOTTOM_ROW: BitBoard = bitboard![0, 0, 0, 0x1fffc0000];
    /// Only the left column.
    pub const LEFT_COL: BitBoard = bitboard![
        0x1000200040008001,
        0x100020004000800,
        0x10002000400080,
        0x40008
    ];
    /// Only the right column.
    pub const RIGHT_COL: BitBoard = bitboard![
        0x800100020004000,
        0x80010002000400,
        0x8001000200040,
        0x100020004
    ];

    /// Gets an iterator over the [`WordBoundaries`](super::words::WordBoundaries)s
    /// on the board.
    pub fn word_boundaries(self) -> WordBoundaries {
        WordBoundaries::new(self)
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
    pub fn words_h(self) -> BitBoard {
        // finds all squares which start a horizontal word
        let starts_h = (self >> 1) & !Self::RIGHT_COL;
        let ends_h = (self << 1) & !Self::LEFT_COL;

        self & (starts_h ^ ends_h)
    }
    /// Calculates the set of tiles with no tile to the immediate left.
    pub fn word_starts_h(self) -> BitBoard {
        self & (!(self << 1) | Self::LEFT_COL)
    }
    /// Calculates the set of tiles with no tile to the immediate right.
    pub fn word_ends_h(self) -> BitBoard {
        self & (!(self >> 1) | Self::RIGHT_COL)
    }

    /// Gets a bitboard containing the set of squares that
    /// are directly above.
    pub fn north(self) -> BitBoard {
        self >> 15
    }
    /// Gets a bitboard containing the set of squares that
    /// are directly below.
    pub fn south(self) -> BitBoard {
        self << 15
    }
    /// Gets a bitboard containing the set of squares that
    /// are directly to the left.
    pub fn west(mut self) -> BitBoard {
        // discard the leftmost column to prevent overflow
        self &= !Self::LEFT_COL;
        self >> 1
    }
    /// Gets a bitboard containing the set of squares that
    /// are directly to the right.
    pub fn east(mut self) -> BitBoard {
        // discard the rightmost column to prevent overflow
        self &= !Self::RIGHT_COL;
        self << 1
    }

    /// Gets a bitboard containing the set of squares that
    /// are adjacent in one of the 4 orthagonal directions,
    /// to the bits in `self`.
    pub fn neighbours(self) -> BitBoard {
        (self.north() | self.south() | self.west() | self.east()) & !self
    }
    /// Gets a bitboard containing the set of squares that
    /// are directly above or below the bits in `self`.
    pub fn above_or_below(self) -> BitBoard {
        (self.north() | self.south()) & !self
    }

    /// Gets the bit at `pos`. `true` if the bit is set.
    pub fn is_set(&self, pos: impl Into<Pos>) -> bool {
        let idx = usize::from(pos.into());
        (self.boards[idx / WORD_SIZE] & (1 << (idx % WORD_SIZE))) != 0
    }
    /// Sets the bit at `pos` to 1.
    pub fn set(&mut self, pos: impl Into<Pos>) {
        let idx = usize::from(pos.into());

        self.boards[idx / WORD_SIZE] |= 1 << (idx % WORD_SIZE);
    }
    /// Sets the bit at `pos` to 0.
    pub fn clear(&mut self, pos: impl Into<Pos>) {
        let idx = usize::from(pos.into());

        self.boards[idx / WORD_SIZE] &= !(1 << (idx % WORD_SIZE));
    }
    /// Checks whether all the bits are set to zero.
    pub fn is_zero(&self) -> bool {
        self.boards.iter().all(|&board| board == 0)
    }
    /// Checks whether the current board and `other` contain any
    /// intersecting bits.
    pub fn intersects(&self, other: &Self) -> bool {
        self.boards
            .iter()
            .zip(other.boards)
            .any(|(&a, b)| a & b != 0)
    }
    /// Counts the number of bits that are set on the board.
    pub fn bit_count(&self) -> usize {
        self.boards.iter().map(|b| b.count_ones() as usize).sum()
    }
    /// Iterates over the bits.
    pub fn iter(self) -> Bits {
        Bits::from(self)
    }
    /// Iterates over the bits in reverse order.
    pub fn iter_reverse(self) -> ReverseBits {
        ReverseBits::from(self)
    }
}

impl IntoIterator for BitBoard {
    type Item = Pos;
    type IntoIter = Bits;

    fn into_iter(self) -> Self::IntoIter {
        Bits::from(self)
    }
}
impl Shl<usize> for BitBoard {
    type Output = Self;

    fn shl(mut self, rhs: usize) -> Self {
        self <<= rhs;
        self
    }
}
impl ShlAssign<usize> for BitBoard {
    fn shl_assign(&mut self, rhs: usize) {
        let words = self.boards.len();

        // for rhs > WORD_SIZE, the shift is equal to `n * WORD_SIZE + k`,
        // where k < WORD_SIZE. This can be carried out as a shift by k,
        // followed by a shift by n * WORD_SIZE. k = rhs % WORD_SIZE.

        // perform shift by k
        let k = rhs % WORD_SIZE;

        if k > 0 {
            // store the remainder from a shift to use in the next shift
            let mut carry = 0;
            for word_idx in 0..words {
                // store the current value
                let tmp = self.boards[word_idx];
                // find the shifted value of the board, and add the carry from
                // the previous iteration
                self.boards[word_idx] = (tmp << k) | carry;
                // find the carry from the shift
                carry = tmp >> (WORD_SIZE - k);
            }
        }

        // perform shift by n * WORD_SIZE
        let n = (rhs / WORD_SIZE) % words;

        // rotate right = shift upwards
        self.boards.rotate_right(n);

        // set all boards below n to zero
        self.boards[0..n].fill(0);

        // fix any extra bits
        self.boards[3] &= FINAL_WORD_MASK;
    }
}
impl Shr<usize> for BitBoard {
    type Output = Self;

    fn shr(mut self, rhs: usize) -> Self {
        self >>= rhs;
        self
    }
}
impl ShrAssign<usize> for BitBoard {
    fn shr_assign(&mut self, rhs: usize) {
        let words = self.boards.len();

        let k = rhs % WORD_SIZE;

        if k > 0 {
            // store the remainder from a shift to use in the next shift
            let mut carry = 0;
            for i in (0..words).rev() {
                // store the current value
                let tmp = self.boards[i];
                // find the shifted value of the board, and add the carry from
                // the previous iteration
                self.boards[i] = (tmp >> k) | carry;
                // find the carry from the shift
                carry = tmp << (WORD_SIZE - k);
            }
        }

        // perform shift by n * WORD_SIZE
        let n = (rhs / WORD_SIZE) % words;

        // rotate left = shift downwards
        self.boards.rotate_left(n);

        // set all boards above and including n to zero
        self.boards[(words - n)..].fill(0);
    }
}
impl BitOr<Self> for BitBoard {
    type Output = Self;

    fn bitor(mut self, rhs: Self) -> Self {
        self |= rhs;
        self
    }
}
impl BitOrAssign<Self> for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.boards
            .iter_mut()
            .zip(rhs.boards)
            .for_each(|(a, b)| *a |= b);
    }
}
impl BitAnd<Self> for BitBoard {
    type Output = Self;

    fn bitand(mut self, rhs: Self) -> Self {
        self &= rhs;
        self
    }
}
impl BitAndAssign<Self> for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.boards
            .iter_mut()
            .zip(rhs.boards)
            .for_each(|(a, b)| *a &= b);
    }
}
impl BitXor<Self> for BitBoard {
    type Output = Self;

    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl BitXorAssign<Self> for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.boards
            .iter_mut()
            .zip(rhs.boards)
            .for_each(|(a, b)| *a ^= b);
    }
}
impl Not for BitBoard {
    type Output = Self;

    fn not(mut self) -> Self {
        let words = self.boards.len();

        for i in 0..words {
            self.boards[i] = !self.boards[i];
        }

        // fix any extra bits
        self.boards[3] &= FINAL_WORD_MASK;

        self
    }
}

impl FromIterator<Pos> for BitBoard {
    fn from_iter<T: IntoIterator<Item = Pos>>(iter: T) -> Self {
        let mut bb = BitBoard::default();

        for pos in iter {
            bb.set(pos);
        }

        bb
    }
}
impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_grid(f, |pos| match self.is_set(pos) {
            true => " x ",
            false => "   ",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not() {
        let bb = bitboard![
            0x00ff_00ff_00ff_00ff_u64,
            0xaa00_aa00_aa00_aa00_u64,
            0x00ff_00ff_00ff_00ff_u64,
            0xaa00_aa00_aa00_aa00_u64 & FINAL_WORD_MASK
        ];
        let not_bb = !bb;
        assert_eq!(
            not_bb.boards,
            [
                0xff00_ff00_ff00_ff00_u64,
                0x55ff_55ff_55ff_55ff_u64,
                0xff00_ff00_ff00_ff00_u64,
                0x55ff_55ff_55ff_55ff_u64 & FINAL_WORD_MASK,
            ]
        );
    }

    #[test]
    fn shift() {
        assert_eq!(BitBoard::TOP_ROW << (15 * 14), BitBoard::BOTTOM_ROW);
        assert_eq!(BitBoard::LEFT_COL << 14, BitBoard::RIGHT_COL);
    }

    #[test]
    fn or() {
        let bb = bitboard![
            0xaa00_aa00_aa00_aa00_u64,
            0x00bb_00bb_00bb_00bb_u64,
            0xaa00_aa00_aa00_aa00_u64,
            0x00bb_00bb_00bb_00bb_u64 & FINAL_WORD_MASK
        ];
        let other = bitboard![
            0x00aa_00aa_00aa_00aa_u64,
            0xbb00_bb00_bb00_bb00_u64,
            0x00aa_00aa_00aa_00aa_u64,
            0xbb00_bb00_bb00_bb00_u64 & FINAL_WORD_MASK
        ];
        let or = bb | other;
        assert_eq!(
            or.boards,
            [
                0xaaaa_aaaa_aaaa_aaaa_u64,
                0xbbbb_bbbb_bbbb_bbbb_u64,
                0xaaaa_aaaa_aaaa_aaaa_u64,
                0xbbbb_bbbb_bbbb_bbbb_u64 & FINAL_WORD_MASK,
            ]
        );
    }

    #[test]
    fn and() {
        let bb = bitboard![
            0xaa00_aa00_aa00_aa00_u64,
            0x00bb_00bb_00bb_00bb_u64,
            0xaa00_aa00_aa00_aa00_u64,
            0x00bb_00bb_00bb_00bb_u64 & FINAL_WORD_MASK
        ];
        let other = bitboard![
            0xaa00_aa00_aa00_aa00_u64,
            0x00bb_00bb_00bb_00bb_u64,
            0xaa00_aa00_aa00_aa00_u64,
            0x00bb_00bb_00bb_00bb_u64 & FINAL_WORD_MASK
        ];
        let and = bb & other;
        assert_eq!(and, bb);
    }

    #[test]
    fn bits() {
        let mut bb = BitBoard::default();
        for pos in Pos::iter() {
            bb.set(pos);
            assert!(bb.is_set(pos));

            bb.clear(pos);
            assert!(bb.is_zero());
        }
    }
}
