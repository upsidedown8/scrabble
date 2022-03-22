//! A simple Ai implementation that just chooses the highest
//! scoring play.

use crate::{
    ai::{
        movegen::{self, ScoredPlay},
        Ai,
    },
    game::{board::Board, play::Play, rack::Rack},
    util::fsm::Fsm,
};

/// An [`Ai`] implementation that chooses the highest scoring
/// play unconditionally.
#[derive(Debug, Default)]
pub struct HighestScoring;

impl Ai for HighestScoring {
    type Difficulty = ();

    fn select_play<'a, F: Fsm<'a>>(
        &self,
        fsm: &'a F,
        board: &Board,
        rack: &Rack,
        _: Self::Difficulty,
    ) -> Play {
        let mut plays = Vec::new();
        movegen::gen(board, rack, fsm, &mut plays);

        plays
            .into_iter()
            .max_by_key(|ScoredPlay(_, score)| *score)
            .map(|ScoredPlay(play, _)| play)
            .unwrap_or(Play::Pass)
    }
}
