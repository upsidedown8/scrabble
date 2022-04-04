use crate::{db::Db, error::Result};

/// A record in `tbl_play`.
#[derive(Debug)]
pub struct Play {
    /// The id of the record.
    pub id_play: i32,
    /// References the record in `tbl_player` that made the play.
    pub id_player: i32,
}

impl Play {
    /// Inserts a play into the database, returning the id.
    pub async fn insert(db: &Db, id_player: i32) -> Result<i32> {
        let id_play = sqlx::query_file_scalar!("sql/live/insert_play.sql", id_player)
            .fetch_one(db)
            .await?;
        Ok(id_play)
    }
}
