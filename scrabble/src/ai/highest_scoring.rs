//! A simple Ai implementation that just chooses the highest
//! scoring play.

use crate::{util::fsm::Fsm, game::{board::Board, rack::Rack, play::Play}, ai::{Ai, movegen::{self, ScoredPlay}}};

/// An [`Ai`] implementation that chooses the highest scoring
/// play unconditionally.
#[derive(Debug, Default)]
pub struct HighestScoring;

impl Ai for HighestScoring {
    type Difficulty = ();

    fn select_play<'a, F: Fsm<'a>>(&self, fsm: &'a F, board: &Board, rack: &Rack, _: Self::Difficulty) -> Play {
        let plays = movegen::gen(board, rack, fsm);

        plays.into_iter()
            .max_by_key(|ScoredPlay(_, score)| *score)
            .map(|ScoredPlay(play, _)| play)
            .unwrap_or(Play::Pass)
    }
}
