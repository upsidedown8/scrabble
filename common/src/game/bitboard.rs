use crate::game::pos::{Col, Pos, Row};
use std::{
    fmt::{Display, Formatter, Result},
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not, Shl, ShlAssign, Shr, ShrAssign},
};

/// [`WORD_SIZE`] = the number of bits in each word. A [`u32`] could have been
/// used giving [`WORD_SIZE`] = 32.
const WORD_SIZE: usize = 64;

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
}

impl Shl<usize> for BitBoard {
    type Output = Self;

    fn shl(mut self, rhs: usize) -> Self {
        self <<= rhs;
        self
    }
}
impl ShlAssign<usize> for BitBoard {
    fn shl_assign(&mut self, mut rhs: usize) {
        let words = self.boards.len();

        // if the shift is greater than the block size, then shift the
        // boards array by rhs DIV 64
        if rhs >= WORD_SIZE {
            // how many 64 bit units to shift by
            let shift_amount = rhs / WORD_SIZE;
            // shift the boards
            self.boards.rotate_left(shift_amount);
            // assign the boards on the right side to zero
            for i in (words - shift_amount)..words {
                self.boards[i] = 0;
            }
            // get the remaining shift
            rhs %= WORD_SIZE;
        }

        // store the remainder from a shift to use in the next shift
        let mut carry = 0;
        for i in (0..words).rev() {
            // store the current value
            let tmp = self.boards[i];
            // find the shifted value of the board, and add the carry from
            // the previous iteration
            self.boards[i] = (tmp << rhs) | carry;
            // find the carry from the shift
            carry = tmp >> (WORD_SIZE - rhs);
        }
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
    fn shr_assign(&mut self, mut rhs: usize) {
        let words = self.boards.len();

        // if the shift is greater than the block size, then shift the
        // boards array by rhs DIV 64
        if rhs >= WORD_SIZE {
            // how many 64 bit units to shift by
            let shift_amount = rhs / WORD_SIZE;
            // shift the boards
            self.boards.rotate_right(shift_amount);
            // assign the boards on the left side to zero
            for i in 0..shift_amount {
                self.boards[i] = 0;
            }
            // get the remaining shift
            rhs %= WORD_SIZE;
        }

        // store the remainder from a shift to use in the next shift
        let mut carry = 0;
        for i in 0..words {
            // store the current value
            let tmp = self.boards[i];
            // find the shifted value of the board, and add the carry from
            // the previous iteration
            self.boards[i] = (tmp >> rhs) | carry;
            // find the carry from the shift
            carry = tmp << (WORD_SIZE - rhs);
        }
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
