//! Scrabble AI implementation.

use crate::{
    game::{board::Board, play::Play, rack::Rack},
    util::fsm::Fsm,
};

pub mod highest_scoring;
pub mod lookup;
pub mod movegen;

/// Trait providing a `select_play` method
pub trait Ai {
    /// A custom type for setting the difficulty level.
    type Difficulty: Default;

    /// Chooses a play based on the position and `difficulty`.
    fn select_play<'a, F: Fsm<'a>>(
        &self,
        fsm: &'a F,
        board: &Board,
        rack: &Rack,
        difficulty: Self::Difficulty,
    ) -> Play;
}
