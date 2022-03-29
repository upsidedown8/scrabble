//! API types for /leaderboard.

use serde::{Deserialize, Serialize};

/// Response from the leaderboard route.
#[derive(Serialize, Deserialize)]
pub struct LeaderboardResponse {
    /// The rows from the route.
    pub rows: Vec<LeaderboardRow>,
}

/// One row of the leaderboard.
#[derive(Serialize, Deserialize)]
pub struct LeaderboardRow {
    /// The username for the entry.
    pub username: String,
    /// Averages score of a play.
    pub avg_score_per_play: f32,
    /// Average length of the longest word placed on each play.
    pub avg_word_length: f32,
    /// Average tiles placed per play.
    pub avg_tiles_per_play: f32,
    /// Longest word placed on a play.
    pub longest_word_length: usize,
    /// Highest score on a play.
    pub best_word_score: usize,
    /// Average overall score per game.
    pub avg_score_per_game: f32,
    /// Average score per tile.
    pub avg_score_per_tile: f32,
    /// Percent of games that the user won.
    pub win_percentage: f32,
}
