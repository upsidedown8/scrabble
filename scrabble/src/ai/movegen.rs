//! Move generator implementation.

use crate::{game::board::Board, util::fsm::Fsm};

/// Generates moves for a board position.
#[derive(Debug, Clone, Copy)]
pub struct MoveGenerator<'a, F> {
    fsm: &'a F,
}

impl<'a, F> MoveGenerator<'a, F>
where
    F: Fsm<'a>,
{
    /// Constructs a new `MoveGenerator`.
    pub fn new(fsm: &'a F) -> Self {
        Self { fsm }
    }
    // /// Returns an iterator over the moves in a position.
    // pub fn moves(&self, board: &Board) -> impl Iterator {
    // }
}
