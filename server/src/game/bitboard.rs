use std::{
    fmt::{Display, Formatter, Result},
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not, Shl, ShlAssign, Shr, ShrAssign},
};

/// Convert from a row and col coordinate to an index from 0..225.
///
/// # Arguments
///
/// * `row` The row from 0..15
/// * `col` The col from 0..15
pub fn get_idx(row: usize, col: usize) -> usize {
    assert!(row < ROWS);
    assert!(col < COLS);
    (row * COLS) + col
}

const ROWS: usize = 15;
const COLS: usize = 15;
const SIZE: usize = 4;
const WORD_SIZE: usize = 64;

/// A scrabble board has 15 * 15 = 225 squares. The nearest multiple
/// of 64 bit integers is 4, giving 256 possible values.
///
/// Using integer types allows for very efficient move generation,
/// validation and scoring, since these operations can be run with a
/// single cpu instruction.
#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub struct BitBoard {
    /// Use 4 unsigned 64 bit integers to represent a 255 bit board
    boards: [u64; SIZE],
}

impl BitBoard {
    /// Checks whether the bit at `idx` is set.
    ///
    /// # Arguments
    ///
    /// * `idx` The position to check
    pub fn is_bit_set<T: Into<usize>>(&self, idx: T) -> bool {
        let idx = idx.into();

        assert!(idx < WORD_SIZE * SIZE);
        (self.boards[idx / WORD_SIZE] & (1u64 << (idx % WORD_SIZE))) != 0
    }

    /// Sets the bit at `idx` to 1.
    ///
    /// # Arguments
    ///
    /// * `idx` The position to set
    pub fn set_bit<T: Into<usize>>(&mut self, idx: T) {
        let idx = idx.into();

        assert!(idx < WORD_SIZE * SIZE);
        self.boards[idx / WORD_SIZE] |= 1u64 << (idx % WORD_SIZE);
    }

    /// Sets the bit at `idx` to 0.
    ///
    /// # Arguments
    ///
    /// * `idx` The position to set
    pub fn clear_bit<T: Into<usize>>(&mut self, idx: T) {
        let idx = idx.into();

        assert!(idx < WORD_SIZE * SIZE);
        self.boards[idx / WORD_SIZE] &= !(1u64 << (idx % WORD_SIZE));
    }

    /// Generates a random bitboard.
    pub fn random() -> BitBoard {
        BitBoard {
            boards: [
                fastrand::u64(..),
                fastrand::u64(..),
                fastrand::u64(..),
                fastrand::u64(..),
            ],
        }
    }

    /// Checks whether all the bits are set to zero.
    pub fn is_zero(&self) -> bool {
        self.boards.iter().all(|&board| board == 0)
    }
}

impl Shl<usize> for BitBoard {
    type Output = BitBoard;

    fn shl(mut self, rhs: usize) -> BitBoard {
        self <<= rhs;
        self
    }
}
impl ShlAssign<usize> for BitBoard {
    fn shl_assign(&mut self, mut rhs: usize) {
        // if the shift is greater than the block size, then shift the
        // boards array by rhs DIV 64
        if rhs >= WORD_SIZE {
            // how many 64 bit units to shift by
            let shift_amount = rhs / WORD_SIZE;
            // shift the boards
            self.boards.rotate_left(shift_amount);
            // assign the boards on the right side to zero
            for i in (SIZE - shift_amount)..SIZE {
                self.boards[i] = 0;
            }
            // get the remaining shift
            rhs %= WORD_SIZE;
        }

        // store the remainder from a shift to use in the next shift
        let mut carry = 0;
        for i in (0..SIZE).rev() {
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
    type Output = BitBoard;

    fn shr(mut self, rhs: usize) -> BitBoard {
        self >>= rhs;
        self
    }
}
impl ShrAssign<usize> for BitBoard {
    fn shr_assign(&mut self, mut rhs: usize) {
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
        for i in 0..SIZE {
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

impl BitOr<BitBoard> for BitBoard {
    type Output = BitBoard;

    fn bitor(mut self, rhs: BitBoard) -> BitBoard {
        self |= rhs;
        self
    }
}
impl BitOrAssign<BitBoard> for BitBoard {
    fn bitor_assign(&mut self, rhs: BitBoard) {
        for i in 0..SIZE {
            self.boards[i] |= rhs.boards[i];
        }
    }
}

impl BitAnd<BitBoard> for BitBoard {
    type Output = BitBoard;

    fn bitand(mut self, rhs: BitBoard) -> BitBoard {
        self &= rhs;
        self
    }
}
impl BitAndAssign<BitBoard> for BitBoard {
    fn bitand_assign(&mut self, rhs: BitBoard) {
        for i in 0..SIZE {
            self.boards[i] &= rhs.boards[i];
        }
    }
}

impl Not for BitBoard {
    type Output = BitBoard;

    fn not(mut self) -> BitBoard {
        for i in 0..SIZE {
            self.boards[i] = !self.boards[i];
        }
        self
    }
}

impl Display for BitBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // column coords
        writeln!(f, "  00 01 02 03 04 05 06 07 08 09 10 11 12 13 14")?;

        // main loop
        for row in 0..ROWS {
            write!(f, "{} ", (row as u8 + 65) as char)?;
            for col in 0..COLS {
                if self.is_bit_set(get_idx(row, col)) {
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
        for idx in 0..225_usize {
            bb.set_bit(idx);
            assert!(bb.is_bit_set(idx));

            bb.clear_bit(idx);
            assert!(bb.is_zero());
        }
    }
}
