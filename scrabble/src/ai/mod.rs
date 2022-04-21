//! Scrabble AI implementation.

use crate::{
    ai::movegen::GeneratedPlay,
    game::{board::Board, play::Play, rack::Rack, Game, GameStatus},
    util::{self, fsm::Fsm},
};
use rand::Rng;

pub mod lookup;
pub mod movegen;

/// The weighting of the proportional length difference in the score.
const LEN_WEIGHT: f32 = 0.8;
/// The weighting of the proportional tile count difference in the score.
const TILES_WEIGHT: f32 = 0.2;
/// The weighting of the score in the final score calculation.
const SCORE_WEIGHT: f32 = 1.0;
/// The weighting of the cross word count in the final score calculation.
const CROSS_WORD_WEIGHT: f32 = 0.8;
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
    /// The ideal score for each play (set to infinity to maximise).
    preferred_score: usize,
    /// If set, makes words of this length more likely to be chosen.
    preferred_len: Option<usize>,
    /// If set, makes plays containing this number of tiles more likely to be chosen.
    preferred_tiles: Option<usize>,
    /// The range of the number of perpendicular words that can be formed in a play.
    preferred_cross_words: Option<usize>,
}

impl Default for Ai {
    fn default() -> Self {
        // The default implementation chooses the highest scored play.
        Self {
            random_factor: 0.0,
            preferred_score: usize::MAX,
            preferred_len: None,
            preferred_tiles: None,
            preferred_cross_words: None,
        }
    }
}
impl Ai {
    /// A preset easy difficulty.
    pub fn easy() -> Self {
        Self {
            random_factor: 0.2,
            preferred_score: 17,
            preferred_len: Some(15),
            preferred_tiles: Some(6),
            preferred_cross_words: Some(0),
        }
    }
    /// A preset medium difficulty.
    pub fn medium() -> Self {
        Self {
            random_factor: 0.1,
            preferred_score: 28,
            preferred_len: Some(8),
            preferred_tiles: Some(5),
            preferred_cross_words: Some(1),
        }
    }
    /// A preset hard difficulty.
    pub fn hard() -> Self {
        Self {
            random_factor: 0.05,
            preferred_score: 50,
            preferred_len: None,
            preferred_tiles: None,
            preferred_cross_words: None,
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
        // Find an initial list of plays.
        let mut plays = vec![];
        movegen::gen(board, rack, fsm, &mut plays);

        // The score should be minimised, so find the play with
        // lowest score.
        let best_play = plays
            .into_iter()
            .min_by(|a, b| {
                let score_a = self.score(a);
                let score_b = self.score(b);

                score_a.partial_cmp(&score_b).unwrap()
            })
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

    /// Takes into account:
    /// * `random_factor`
    /// * `preferred_len`
    /// * `preferred_tiles`
    /// * `preferred_cross_words`
    /// and the score of the word to calculate a new score, which
    /// should be minimised.
    fn score(&self, gen_play: &GeneratedPlay) -> f32 {
        // find the difference between the word length and preferred length
        let len_score = self
            .preferred_len
            .map(|pl| util::abs_diff(pl, gen_play.len))
            .map(|diff| LEN_WEIGHT * (diff as f32))
            .unwrap_or(0.0);
        // find the difference between the tile count and preferred count
        let tiles_score = self
            .preferred_tiles
            .map(|pt| util::abs_diff(pt, gen_play.tile_positions.len()))
            .map(|diff| TILES_WEIGHT * (diff as f32))
            .unwrap_or(0.0);
        // find the difference between the cross word count and preferred count.
        let cross_word_score = self
            .preferred_cross_words
            .map(|pcw| util::abs_diff(pcw, gen_play.cross_count))
            .map(|diff| CROSS_WORD_WEIGHT * (diff as f32))
            .unwrap_or(0.0);
        // find the difference between the actual score and the preferred score.
        let score_diff = util::abs_diff(self.preferred_score, gen_play.score) as f32;

        // smaller is better for `len_diff`, `tiles_diff` and `score_diff`, so
        // combine them by taking the reciprocal of each. (1.0 is added to each
        // to avoid a zero division error). Each reciprocal is multiplied by a
        // constant weight factor to fine tune the evaulation function.
        let combined_score = cross_word_score + len_score + tiles_score + SCORE_WEIGHT * score_diff;

        // apply a final random factor to the score.
        let multiplier = rand::thread_rng().gen_range(-1.0..1.0);
        combined_score * (1.0 + multiplier * self.random_factor)
    }
}
