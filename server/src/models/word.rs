use crate::{db::Db, error::Result};

/// A record in `tbl_word`.
#[derive(Debug)]
pub struct Word {
    /// Autoincrementing id for the record.
    pub id_word: i32,
    /// References the play in which the word was placed.
    pub id_play: i32,
    /// The score of the word.
    pub score: i32,
    /// The letters of the word.
    pub letters: String,
}

impl Word {
    /// Inserts a word into the database.
    pub async fn insert(db: &Db, id_play: i32, letters: String, score: usize) -> Result<()> {
        sqlx::query_file!("sql/live/insert_word.sql", id_play, letters, score as i32)
            .execute(db)
            .await?;
        Ok(())
    }
}
