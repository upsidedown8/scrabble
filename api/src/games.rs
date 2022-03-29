//! API types for existing games.

use serde::{Deserialize, Serialize};

/// Body of the request to create a game.
#[derive(Serialize, Deserialize)]
pub struct CreateGame {
    /// The number of human players in the game.
    pub player_count: usize,
    /// The number of ai player in the game,
    pub ai_count: usize,
}

/// Response from creating a game.
#[derive(Serialize, Deserialize)]
pub struct CreateGameResponse {
    /// The id of the game to join.
    pub id_game: usize,
}
