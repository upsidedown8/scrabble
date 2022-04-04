use crate::{error::Result, Db};
use chrono::{NaiveDateTime, Utc};

/// A record in `tbl_game`.
#[derive(Debug)]
pub struct Game {
    /// Id for the game.
    pub id_game: i32,
    /// The start time of the game.
    pub start_time: Option<NaiveDateTime>,
    /// The end time of the game.
    pub end_time: Option<NaiveDateTime>,
    /// Whether the game is over.
    pub is_over: bool,
}

impl Game {
    /// Inserts the record into the database, returning the id.
    pub async fn insert(db: &Db) -> Result<i32> {
        let start_time = Some(Utc::now().naive_utc());

        let id_game = sqlx::query_file_scalar!(
            "sql/live/insert_game.sql",
            start_time,
            Option::<NaiveDateTime>::None, // end_time
            false,                         // is_over
        )
        .fetch_one(db)
        .await?;

        Ok(id_game)
    }
}
