use crate::{db::Db, error::Result};
use scrabble::{game::tile, util::pos::Pos};

/// A record in `tbl_tile`.
#[derive(Debug, Clone)]
pub struct Tile {
    /// Foreign key to the play in which this tile was placed.
    pub id_play: i32,
    /// The position on which the tile was placed.
    pub pos: i32,
    /// The letter that was placed.
    pub letter: char,
    /// Whether the tile was blank.
    pub is_blank: bool,
}

impl Tile {
    /// Inserts a tile into the database.
    pub async fn insert(db: &Db, id_play: i32, pos: &Pos, tile: &tile::Tile) -> Result<()> {
        let pos = usize::from(*pos) as i32;
        let letter = char::from(tile.letter().unwrap());
        let is_blank = tile.is_blank();

        sqlx::query_file!(
            "sql/live/insert_tile.sql",
            id_play,
            pos,
            letter.to_string(),
            is_blank
        )
        .execute(db)
        .await?;

        Ok(())
    }
}
