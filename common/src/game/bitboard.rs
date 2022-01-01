//! Module containing a bitboard implementation to represent
//! the occupancy on the 15 x 15 board.

use crate::game::pos::{Col, Pos, Row};
use std::{
    fmt::{Display, Formatter, Result},
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not, Shl, ShlAssign, Shr, ShrAssign},
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
                let rev = word.reverse_bits();
                let trailing_zeros = rev.leading_zeros() as usize;
                self.boards[self.word_idx] &= !(1 << trailing_zeros);

                return Some(Pos::from(trailing_zeros + WORD_SIZE * self.word_idx));
            }
        }

        None
    }
}

/// A scrabble board has [`ROWS`] * [`COLS`] = 15 * 15 = 225 squares. The
/// nearest multiple of 64 bit integers is 4, giving 256 bit values.
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
    /// A bitboard with all bits set to 0.
    pub const fn zero() -> Self {
        Self { boards: [0; 4] }
    }
    /// A bitboard with all bits set to 1.
    pub const fn full() -> Self {
        Self {
            boards: [u64::MAX, u64::MAX, u64::MAX, FINAL_WORD_MASK],
        }
    }
    /// A bitboard where the top row is set to 1.
    pub const fn top_row() -> Self {
        Self {
            boards: [32767, 0, 0, 0],
        }
    }
    /// A bitboard where the bottom row is set to 1.
    pub const fn bottom_row() -> Self {
        Self {
            boards: [0, 0, 0, 8589672448],
        }
    }
    /// A bitboard where the leftmost row is set to 1.
    pub const fn leftmost_col() -> Self {
        Self {
            boards: [
                1152956690052710401,
                72059793128294400,
                4503737070518400,
                262152,
            ],
        }
    }
    /// A bitboard where the leftmost row is set to 1.
    pub const fn rightmost_col() -> Self {
        Self {
            boards: [
                576478345026355200,
                36029896564147200,
                2251868535259200,
                4295098372,
            ],
        }
    }

    /// Checks whether the bit at `pos` is set.
    pub fn is_bit_set<T: Into<Pos>>(&self, pos: T) -> bool {
        let idx = usize::from(pos.into());

        (self.boards[idx / WORD_SIZE] & (1 << (idx % WORD_SIZE))) != 0
    }
    /// Sets the bit at `pos` to 1.
    pub fn set_bit<T: Into<Pos>>(&mut self, pos: T) {
        let idx = usize::from(pos.into());

        self.boards[idx / WORD_SIZE] |= 1 << (idx % WORD_SIZE);
    }
    /// Sets the bit at `pos` to 0.
    pub fn clear_bit<T: Into<Pos>>(&mut self, pos: T) {
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
}

impl IntoIterator for BitBoard {
    type Item = Pos;
    type IntoIter = Bits;

    fn into_iter(self) -> Self::IntoIter {
        Bits {
            boards: self.boards,
            word_idx: 0,
        }
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
        let words = self.boards.len();

        for i in 0..words {
            self.boards[i] |= rhs.boards[i];
        }
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
        let words = self.boards.len();

        for i in 0..words {
            self.boards[i] &= rhs.boards[i];
        }
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

impl Display for BitBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // print col headers
        write!(f, "   ")?;
        for col in Col::iter() {
            write!(f, " {} ", col)?;
        }
        writeln!(f)?;

        // main loop
        for row in Row::iter() {
            write!(f, "{:>2} ", row.to_string())?;
            for col in Col::iter() {
                if self.is_bit_set((row, col)) {
                    write!(f, " x ")?;
                } else {
                    write!(f, "   ")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not() {
        let bb = BitBoard {
            boards: [
                0x00ff_00ff_00ff_00ff_u64,
                0xaa00_aa00_aa00_aa00_u64,
                0x00ff_00ff_00ff_00ff_u64,
                0xaa00_aa00_aa00_aa00_u64,
            ],
        };
        let not_bb = !bb;
        assert_eq!(
            not_bb.boards,
            [
                0xff00_ff00_ff00_ff00_u64,
                0x55ff_55ff_55ff_55ff_u64,
                0xff00_ff00_ff00_ff00_u64,
                0x55ff_55ff_55ff_55ff_u64,
            ]
        );
    }

    #[test]
    fn shl() {
        let mut bb = BitBoard {
            boards: [
                0x00ff_00ff_00ff_00ff_u64,
                0xaa00_aa00_aa00_aa00_u64,
                0x00ff_00ff_00ff_00ff_u64,
                0xaa00_aa00_aa00_aa00_u64,
            ],
        };
        bb <<= 8;
        assert_eq!(
            bb.boards,
            [
                0xff00_ff00_ff00_ffaa_u64,
                0x00aa_00aa_00aa_0000_u64,
                0xff00_ff00_ff00_ffaa_u64,
                0x00aa_00aa_00aa_0000_u64,
            ]
        );
    }

    #[test]
    fn shr() {
        let mut bb = BitBoard {
            boards: [
                0x00ff_00ff_00ff_00ff_u64,
                0xaa00_aa00_aa00_aa00_u64,
                0x00ff_00ff_00ff_00ff_u64,
                0xaa00_aa00_aa00_aa00_u64,
            ],
        };

        bb >>= 8;
        assert_eq!(
            bb.boards,
            [
                0x0000_ff00_ff00_ff00_u64,
                0xffaa_00aa_00aa_00aa_u64,
                0x0000_ff00_ff00_ff00_u64,
                0xffaa_00aa_00aa_00aa_u64,
            ]
        );
    }

    #[test]
    fn or() {
        let bb = BitBoard {
            boards: [
                0xaa00_aa00_aa00_aa00_u64,
                0x00bb_00bb_00bb_00bb_u64,
                0xaa00_aa00_aa00_aa00_u64,
                0x00bb_00bb_00bb_00bb_u64,
            ],
        };
        let other = BitBoard {
            boards: [
                0x00aa_00aa_00aa_00aa_u64,
                0xbb00_bb00_bb00_bb00_u64,
                0x00aa_00aa_00aa_00aa_u64,
                0xbb00_bb00_bb00_bb00_u64,
            ],
        };
        let or = bb | other;
        assert_eq!(
            or.boards,
            [
                0xaaaa_aaaa_aaaa_aaaa_u64,
                0xbbbb_bbbb_bbbb_bbbb_u64,
                0xaaaa_aaaa_aaaa_aaaa_u64,
                0xbbbb_bbbb_bbbb_bbbb_u64,
            ]
        );
    }

    #[test]
    fn and() {
        let bb = BitBoard {
            boards: [
                0xaa00_aa00_aa00_aa00_u64,
                0x00bb_00bb_00bb_00bb_u64,
                0xaa00_aa00_aa00_aa00_u64,
                0x00bb_00bb_00bb_00bb_u64,
            ],
        };
        let other = BitBoard {
            boards: [
                0xaa00_aa00_aa00_aa00_u64,
                0x00bb_00bb_00bb_00bb_u64,
                0xaa00_aa00_aa00_aa00_u64,
                0x00bb_00bb_00bb_00bb_u64,
            ],
        };
        let and = bb & other;
        assert_eq!(and, bb);
    }

    #[test]
    fn bits() {
        let mut bb = BitBoard::default();
        for pos in Pos::iter() {
            bb.set_bit(pos);
            assert!(bb.is_bit_set(pos));

            bb.clear_bit(pos);
            assert!(bb.is_zero());
        }
    }
}
