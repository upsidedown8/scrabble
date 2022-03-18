//! Module for scoring a play.

use crate::{
    error::{GameError, GameResult},
    game::board::Board,
    util::{bitboard::BitBoard, fsm::Fsm, words::Word},
};

/// Validates a word and finds its score.
pub fn score<'a>(
    word: Word,
    new: &BitBoard,
    board: &Board,
    fsm: &impl Fsm<'a>,
) -> GameResult<usize> {
    let mut score = 0;
    let mut word_multiplier = 1;
    let mut curr_state = fsm.initial_state();

    for pos in word {
        let tile = board.get(pos).expect("An occupied square");
        let letter = tile.letter().expect("A letter");

        curr_state = fsm
            .traverse_from(curr_state, letter)
            .ok_or(GameError::InvalidWord)?;

        let (tile_m, word_m) = match new.is_bit_set(pos) {
            true => pos.premium_multipliers(),
            false => (1, 1),
        };

        word_multiplier *= word_m;
        score += tile_m * tile.score();
    }

    match fsm.is_terminal(curr_state) {
        true => Ok(word_multiplier * score),
        false => Err(GameError::InvalidWord),
    }
}
