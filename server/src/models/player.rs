use crate::{db::Db, error::Result};
use std::{convert::Infallible, fmt, str::FromStr};

/// A record in `tbl_friend_request`.
#[derive(Debug, Clone)]
pub struct Player {
    /// Autoincrementing player id.
    pub id_player: i32,
    /// Id of the game the player is participating in,
    pub id_game: i32,
    /// Id of the user. (If `None` then the player is an ai).
    pub id_user: Option<i32>,
    /// Difficulty setting of the ai (easy | medium | hard). Only set
    /// if `id_user` is not set.
    pub ai_difficulty: Option<AiDifficulty>,
    /// Whether the player won the game (may be null).
    pub is_winner: Option<bool>,
}

impl Player {
    /// Inserts an Ai player, returning the id.
    pub async fn insert_ai(db: &Db, id_game: i32, ai_difficulty: AiDifficulty) -> Result<i32> {
        let ai_difficulty = ai_difficulty.to_string();

        let id_player = sqlx::query_file_scalar!(
            "sql/live/insert_ai_player.sql",
            id_game,
            Some(ai_difficulty)
        )
        .fetch_one(db)
        .await?;
        Ok(id_player)
    }
    /// Inserts a user player, returning (id, username).
    pub async fn insert_user(
        db: &Db,
        id_game: i32,
        id_user: i32,
        id_owner: Option<i32>,
    ) -> Result<(i32, String)> {
        // find the username for the user.
        let username = sqlx::query_file!("sql/live/find_username.sql", id_user)
            .fetch_one(db)
            .await?
            .username;

        // id of the inserted player.
        let id_player = match id_owner {
            // Ensure the user is the owner or a friend of the
            // owner.
            Some(id_owner) => {
                sqlx::query_file_scalar!(
                    "sql/live/insert_player_if_friend.sql",
                    id_game,
                    id_user,
                    id_owner
                )
                .fetch_one(db)
                .await?
            }
            // Don't check that the user is the owner.
            None => {
                sqlx::query_file_scalar!("sql/live/insert_player.sql", id_game, id_user)
                    .fetch_one(db)
                    .await?
            }
        };

        Ok((id_player, username))
    }
}

/// Gets the difficult setting of the ai player.
#[derive(Debug, Clone, Copy)]
pub enum AiDifficulty {
    Easy,
    Medium,
    Hard,
}

impl fmt::Display for AiDifficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AiDifficulty::Easy => write!(f, "easy"),
            AiDifficulty::Medium => write!(f, "medium"),
            AiDifficulty::Hard => write!(f, "hard"),
        }
    }
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
