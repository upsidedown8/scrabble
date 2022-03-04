//! Scrabble AI implementation.

use crate::game::{play::Play, board::Board};

pub mod movegen;

/// Serves as a common interface over AI implementations.
pub trait Ai {
    /// Finds the best move from a given position.
    fn best_move(board: &Board) -> Play;
}
