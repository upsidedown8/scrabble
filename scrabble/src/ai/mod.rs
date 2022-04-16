//! Scrabble AI implementation.

use crate::{
    ai::movegen::GeneratedPlay,
    game::{board::Board, play::Play, rack::Rack, Game, GameStatus},
    util::{self, fsm::Fsm},
};
use rand::Rng;
use std::ops::RangeInclusive;

pub mod lookup;
pub mod movegen;

/// The number of top scoring moves to look ahead from.
const LOOK_AHEAD_LIMIT: usize = 15;
/// The weighting of the proportional length difference in the score.
const LEN_WEIGHT: f32 = 12.0;
/// The weighting of the proportional tile count difference in the score.
const TILES_WEIGHT: f32 = 8.0;
/// The weighting of the score in the final score calculation.
const SCORE_WEIGHT: f32 = 10.0;
/// If there are no plays, tiles with fewer than
const REDRAW_LIMIT: usize = 8;

/// An Ai implementation that can play at varying difficulty levels.
#[derive(Debug)]
pub struct Ai {
    /// A number for randomising final move choices. The final score is:
    ///     `score` * (1 + (`rand(-1..1)` * `random_factor`)).
    /// The `random_factor` can be thought of as an uncertainty for the final
    /// score.
    random_factor: f32,
    /// If set, makes words of this length more likely to be chosen.
    preferred_len: Option<usize>,
    /// If set, makes plays containing this number of tiles more likely to be chosen.
    preferred_tiles: Option<usize>,
    /// The ideal score for each play (set to infinity to maximise).
    preferred_score: usize,
    /// The range of the number of tiles that can be placed on a play.
    tile_range: RangeInclusive<usize>,
    /// The range of the number of perpendicular words that can be formed in a play.
    cross_word_range: RangeInclusive<usize>,
    /// Sets the allowed lengths of the primary word.
    len_range: RangeInclusive<usize>,
}

impl Default for Ai {
    fn default() -> Self {
        Self {
            random_factor: 0.0,
            preferred_len: None,
            preferred_tiles: None,
            preferred_score: usize::MAX,
            cross_word_range: 0..=7,
            tile_range: 1..=7,
            len_range: 2..=15,
        }
    }
}
impl Ai {
    /// A preset easy difficulty.
    pub fn easy() -> Self {
        Self {
            random_factor: 0.2,
            preferred_len: Some(6),
            preferred_tiles: Some(5),
            preferred_score: 20,
            cross_word_range: 0..=1,
            tile_range: 1..=6,
            len_range: 3..=15,
        }
    }
    /// A preset medium difficulty.
    pub fn medium() -> Self {
        Self {
            random_factor: 0.15,
            preferred_len: Some(7),
            preferred_tiles: None,
            preferred_score: 30,
            cross_word_range: 0..=2,
            tile_range: 1..=7,
            len_range: 2..=10,
        }
    }
    /// A preset hard difficulty.
    pub fn hard() -> Self {
        Self {
            random_factor: 0.05,
            preferred_len: None,
            preferred_tiles: None,
            preferred_score: 50,
            cross_word_range: 0..=4,
            tile_range: 1..=7,
            len_range: 2..=15,
        }
    }
    /// A setting that always chooses the longest word.
    pub fn longest_word() -> Self {
        Self {
            preferred_len: Some(15),
            ..Ai::default()
        }
    }
    /// A setting that always chooses the highest scoring play.
    pub fn highest_scoring() -> Self {
        Ai::default()
    }

    /// Chooses a play based on the position and ai settings.
    pub fn select_play<'a, F: Fsm<'a>>(
        &self,
        fsm: &'a F,
        board: &Board,
        rack: &Rack,
        letter_bag_len: usize,
    ) -> Play {
        // Find an initial list of scored plays.
        let scored = self.initial_scored_plays(board, rack, fsm);

        // Find the play with highest score.
        let best_play = scored
            .into_iter()
            .max_by(|(first_score, _), (second_score, _)| {
                first_score.partial_cmp(second_score).unwrap()
            })
            .map(|(_, gen_play)| gen_play)
            .map(Play::from);

        // Return the play with highest final score.
        match best_play {
            // If there is a play, return it.
            Some(play) => play,
            // If the rack has fewer than 7 tiles, always pass.
            None if rack.len() < 7 => Play::Pass,
            // Only redraw if we have all 7 tiles.
            None => {
                // Find all rack tiles below the limit (taking no more than are left in
                // the bag).
                let redraw = rack
                    .tiles()
                    .filter(|tile| tile.score() <= REDRAW_LIMIT)
                    .take(letter_bag_len)
                    .collect::<Vec<_>>();

                // Pass if there are no tiles to redraw.
                match redraw.len() {
                    0 => Play::Pass,
                    _ => Play::Redraw(redraw),
                }
            }
        }
    }
    /// Chooses a play for the next player in the game based on
    /// the board position and `difficulty`.
    pub fn next_play<'a, F: Fsm<'a>>(&self, fsm: &'a F, game: &Game) -> Play {
        let &to_play = match game.status() {
            GameStatus::ToPlay(to_play) => to_play,
            GameStatus::Over(_) => panic!("game is over"),
        };
        let rack = game.player(to_play).rack();

        self.select_play(fsm, game.board(), rack, game.letter_bag_len())
    }

    /// Checks whether the cross word count, word length and tile count
    /// fit the difficulty settings.
    fn matches_filter(&self, gen_play: &GeneratedPlay) -> bool {
        let cross_count = gen_play.cross_count;
        let len = gen_play.len;
        let tiles = gen_play.tile_positions.len();

        // must have the correct number of cross words.
        self
            .cross_word_range
            .contains(&cross_count)
        // must have the correct number of tiles.
        && self
                .tile_range
                .contains(&tiles)
        // must be the correct word length.
        && self.len_range.contains(&len)
    }
    /// Takes into account:
    /// * `random_factor`
    /// * `preferred_len`
    /// * `preferred_tiles`
    /// and the score of the word to calculate a new score.
    fn score(&self, gen_play: &GeneratedPlay) -> f32 {
        // find the difference between the word length and preferred length
        let len_score = self
            .preferred_len
            .map(|pl| util::abs_diff(pl, gen_play.len))
            .map(|diff| LEN_WEIGHT / (diff as f32 + 1.0))
            .unwrap_or(0.0);
        // find the difference between the tile count and preferred count
        let tiles_score = self
            .preferred_tiles
            .map(|pt| util::abs_diff(pt, gen_play.tile_positions.len()))
            .map(|diff| TILES_WEIGHT / (diff as f32 + 1.0))
            .unwrap_or(0.0);
        // find the difference between the actual score and the preferred score.
        let score_diff = util::abs_diff(self.preferred_score, gen_play.score) as f32;

        // smaller is better for `len_diff`, `tiles_diff` and `score_diff`, so
        // combine them by taking the reciprocal of each. (1.0 is added to each
        // to avoid a zero division error). Each reciprocal is multiplied by a
        // constant weight factor to fine tune the evaulation function.
        let combined_score = len_score + tiles_score + (SCORE_WEIGHT / (score_diff + 1.0));

        // apply a final random factor to the score.
        let multiplier = rand::thread_rng().gen_range(-1.0..1.0);
        combined_score * (1.0 + multiplier * self.random_factor)
    }
    /// Calculates an initial list of scored generated plays.
    fn initial_scored_plays<'a, F: Fsm<'a>>(
        &self,
        board: &Board,
        rack: &Rack,
        fsm: &'a F,
    ) -> Vec<(f32, GeneratedPlay)> {
        let mut plays = vec![];
        movegen::gen(board, rack, fsm, &mut plays);

        // Immediately remove all plays that do not fit the
        // difficulty conditions.
        plays.retain(|gen_play| self.matches_filter(gen_play));

        // Compute weighted scores for each play. This also applies a random
        // factor to each score.
        let mut scored: Vec<_> = plays
            .into_iter()
            .map(|gen_play| (self.score(&gen_play), gen_play))
            .collect();

        // Sort by highest score.
        scored.sort_unstable_by(|(first_score, _), (second_score, _)| {
            // Have to use `partial_cmp` as `f32` could be `NaN`.
            first_score.partial_cmp(second_score).unwrap()
        });
        // Keep a fixed number for further evaluation
        scored.truncate(LOOK_AHEAD_LIMIT);

        scored
    }
}
