use chrono::{DateTime, Utc};

/// A record in `tbl_game`.
#[derive(Debug)]
pub struct Game {
    /// Uuid as a string for the game.
    pub id_game: String,
    /// The start time of the game.
    pub start_time: Option<DateTime<Utc>>,
    /// The end time of the game.
    pub end_time: Option<DateTime<Utc>>,
    /// Whether the game is over.
    pub is_over: bool,
}
