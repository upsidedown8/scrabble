use std::{convert::Infallible, str::FromStr};

/// A record in `tbl_friend_request`.
#[derive(Debug, Clone)]
pub struct Player {
    /// Autoincrementing player id.
    pub id_player: usize,
    /// Id of the game the player is participating in,
    pub id_game: usize,
    /// Id of the user. (If `None` then the player is an ai).
    pub id_user: Option<usize>,
    /// Difficulty setting of the ai (easy | medium | hard). Only set
    /// if `id_user` is not set.
    pub ai_difficulty: Option<AiDifficulty>,
    /// The initial letters on the player's rack.
    pub initial_rack: String,
    /// Whether the player won the game (may be null).
    pub is_winner: Option<bool>,
}

/// Gets the difficult setting of the ai player.
#[derive(Debug, Clone, Copy)]
pub enum AiDifficulty {
    Easy,
    Medium,
    Hard,
}

impl FromStr for AiDifficulty {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "hard" => Self::Hard,
            "medium" => Self::Medium,
            _ => Self::Easy,
        })
    }
}
