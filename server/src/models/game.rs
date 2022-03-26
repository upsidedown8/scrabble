use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Models a record of `tbl_game`.
#[derive(Debug, Serialize, Deserialize)]
pub struct GameModel {
    /// The id for the record.
    pub id_game: Uuid,
    /// The start time of the game.
    pub start: DateTime<Utc>,
    /// The end time of the game.
    pub end: DateTime<Utc>,
    /// Whether the game is over.
    pub is_over: bool,
}
