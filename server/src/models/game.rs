use crate::{error::Result, Db};
use chrono::NaiveDateTime;

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
    /// Finds the game in the database by id, if the user was
    /// part of the game,
    pub async fn find_by_id_and_user(db: &Db, id_user: i32, id_game: i32) -> Result<Self> {
        let game =
            sqlx::query_file_as!(Game, "sql/games/find_by_id_and_user.sql", id_user, id_game)
                .fetch_one(db)
                .await?;
        Ok(game)
    }
}
