use crate::models::AiDifficulty;
use api::routes::live::{Player, ServerMsg};
use scrabble::game::PlayerNum;
use tokio::sync::mpsc;

/// One of a fixed number of slots in a game.
#[derive(Debug)]
pub struct Slot {
    /// Database id of the player in the slot.
    pub id_player: i32,
    /// The player number in the game representation.
    pub player_num: PlayerNum,
    /// The actual player, either Ai or a connected user.
    pub game_player: GamePlayer,
}

impl Slot {
    /// Checks whether the contained player is a user with the
    /// provided id.
    pub fn has_id_user(&self, id_user: i32) -> bool {
        Some(id_user) == self.id_user()
    }
    /// Gets the optional user id.
    pub fn id_user(&self) -> Option<i32> {
        match self.game_player {
            GamePlayer::User { id_user, .. } => Some(id_user),
            _ => None,
        }
    }
    /// Gets the API Player type for an occupied slot.
    pub fn player(&self) -> Player {
        Player {
            id_player: self.id_player,
            username: match &self.game_player {
                GamePlayer::Ai { difficulty, .. } => String::from(match difficulty {
                    AiDifficulty::Easy => "AI (easy)",
                    AiDifficulty::Medium => "AI (medium)",
                    AiDifficulty::Hard => "AI (hard)",
                }),
                GamePlayer::User {
                    username, sender, ..
                } => match sender.is_some() {
                    true => username.clone(),
                    false => format!("Disconnected ({username})"),
                },
            },
        }
    }
}

/// Either an AI player or a connected user.
#[derive(Debug)]
pub enum GamePlayer {
    /// The player is an AI.
    Ai {
        /// The Ai difficulty.
        difficulty: AiDifficulty,
    },
    /// The player is a connected user.
    User {
        /// Id of the connected user.
        id_user: i32,
        /// Username of the player.
        username: String,
        /// Sender half of an mpsc queue that sends `ServerMsg`s to
        /// a connected user. If `None` the player has disconnected,
        /// and an easy AI will make their moves until they reconnect.
        sender: Option<mpsc::UnboundedSender<ServerMsg>>,
    },
}
