//! Scrabble AI implementation.

use crate::{game::{play::Play, board::Board, rack::Rack}, util::fsm::Fsm};

pub mod movegen;
pub mod lookup;
pub mod highest_scoring;

/// Trait providing a `select_play` method 
pub trait Ai {
    /// A custom type for setting the difficulty level.
    type Difficulty: Default;

    /// Chooses a play based on the position and `difficulty`.
    fn select_play<'a, F: Fsm<'a>>(&self, fsm: &'a F, board: &Board, rack: &Rack, difficulty: Self::Difficulty) -> Play;
}
