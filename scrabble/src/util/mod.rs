//! Module containing functions and structs which are used across
//! the library.

use self::pos::{Col, Pos, Row};
use std::fmt;

pub mod bitboard;
pub mod fsm;
pub mod pos;
pub mod tile_counts;
pub mod word_boundaries;

/// Utility function for displaying a grid, which prints row
/// and column headers. `at_pos` should return a string of length
/// 3 which represents the cell at the provided position.
///
/// This function is used for implementing [`fmt::Display`] for [`Board`]
/// and [`BitBoard`].
pub fn write_grid<F, T>(f: &mut fmt::Formatter, at_pos: F) -> fmt::Result
where
    F: Fn(Pos) -> T,
    T: fmt::Display,
{
    fn write_col_headers(f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "   ")?;
        for col in Col::iter() {
            write!(f, " {} ", col)?;
        }

        Ok(())
    }

    write_col_headers(f)?;

    writeln!(f)?;

    for row in Row::iter() {
        write!(f, "{:>2} ", row.to_string())?;

        for col in Col::iter() {
            write!(f, "{}", at_pos(Pos::from((row, col))))?;
        }

        writeln!(f, " {:<2}", row.to_string())?;
    }

    write_col_headers(f)
}
