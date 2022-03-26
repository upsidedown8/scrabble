use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Models a record of `tbl_game`.
#[derive(Debug, Serialize, Deserialize)]
pub struct GameModel {
    /// The id for the record.
    pub id_game: usize,
    /// The start time of the game.
    pub start: DateTime<Utc>,
    /// The end time of the game.
    pub end: DateTime<Utc>,
    /// Whether the game is over.
    pub is_over: bool,
}
