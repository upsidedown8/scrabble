//! API types for /games.

use std::collections::HashMap;

use crate::routes::leaderboard::LeaderboardRow;
use chrono::NaiveDateTime;
use scrabble::game::play::Play;
use serde::{Deserialize, Serialize};

/// Response from the list games route.
#[derive(Debug, Serialize, Deserialize)]
pub struct ListGamesResponse {
    /// A list of metadata about a user's games.
    pub games: Vec<GameMetadata>,
}

/// Metadata about a game.
#[derive(Debug, Serialize, Deserialize)]
pub struct GameMetadata {
    /// The id of the game.
    pub id_game: i32,
    /// The time that the game started.
    pub start_time: Option<NaiveDateTime>,
    /// The time that the game ended.
    pub end_time: Option<NaiveDateTime>,
    /// Whether the game is over.
    pub is_over: bool,
}

/// Response from the game details route.
#[derive(Debug, Serialize, Deserialize)]
pub struct GameResponse {
    /// Metadata about the game,
    pub meta: GameMetadata,
    /// Ids and usernames of the players of the game.
    pub players: HashMap<i32, String>,
    /// (playerid, play) tuples.
    pub plays: Vec<(i32, Play)>,
}

/// Stats about an individual game.
#[derive(Debug, Serialize, Deserialize)]
pub struct GameStatsResponse {
    /// Metadata about the game.
    pub meta: GameMetadata,
    /// Average score per play.
    pub avg_score_per_play: f32,
    /// Average longest word length of each play.
    pub avg_word_length: f32,
    /// Average words per play.
    pub avg_words_per_play: f32,
    /// Average tile count per play.
    pub avg_tiles_per_play: f32,
    /// Longest word placed.
    pub longest_word_length: usize,
    /// Highest scoring play.
    pub best_word_score: usize,
    /// Average score per tile.
    pub avg_score_per_tile: f32,
    /// Whether the game is a win.
    pub is_win: bool,
}

/// A single leaderboard row for the user.
#[derive(Debug, Serialize, Deserialize)]
pub struct OverallStatsResponse {
    /// The row of the leaderboard.
    pub row: LeaderboardRow,
}
