//! Module for scoring a play.

use crate::{
    error::{GameError, GameResult},
    game::board::Board,
    util::{
        bitboard::BitBoard,
        fsm::Fsm,
        pos::{Direction, Pos},
        words::Word,
    },
};

/// Validates a word and finds its score.
pub fn score<'a>(
    word: Word,
    new: &BitBoard,
    board: &Board,
    fsm: &impl Fsm<'a>,
) -> GameResult<usize> {
    let perpendicular = word.dir().perpendicular();
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

        let cross_word_score = score_crossword(pos, perpendicular, board, fsm)?;

        word_multiplier *= word_m;
        score += tile_m * tile.score() + cross_word_score;
    }

    match fsm.is_terminal(curr_state) {
        true => Ok(word_multiplier * score),
        false => Err(GameError::InvalidWord),
    }
}

/// Finds the score for a word going in the provided direction,
/// at the provided position. Ensures that cross words formed are
/// valid.
fn score_crossword<'a>(
    pos: Pos,
    dir: Direction,
    board: &Board,
    fsm: &impl Fsm<'a>,
) -> GameResult<usize> {
    // check whether there is actually a crossword at this position.
    let has_tile_right = pos.dir(dir).and_then(|p| board.get(p)).is_some();
    let has_tile_left = pos.dir(dir.opposite()).and_then(|p| board.get(p)).is_some();
    if !has_tile_right && !has_tile_left {
        return Ok(0);
    }

    // First traverse to the beginning of the word.
    let opp = dir.opposite();
    let mut curr_pos = pos;
    while let Some((prev_pos, Some(_))) = curr_pos.dir(opp).map(|pos| (pos, board.get(pos))) {
        curr_pos = prev_pos;
    }

    let (tile_m, word_m) = pos.premium_multipliers();

    // Now traverse through the word to determine the score.
    let mut score = 0;
    let mut curr_state = fsm.initial_state();
    loop {
        match curr_pos.dir(dir).map(|pos| (pos, board.get(pos))) {
            Some((next_pos, Some(tile))) => {
                let letter = tile.letter().unwrap();

                curr_state = fsm
                    .traverse_from(curr_state, letter)
                    .ok_or(GameError::InvalidWord)?;

                let t_multiplier = match curr_pos == pos {
                    true => tile_m,
                    false => 1,
                };

                score += tile.score() * t_multiplier;
                curr_pos = next_pos;
            }
            _ => return Ok(score * word_m),
        }
    }
}
