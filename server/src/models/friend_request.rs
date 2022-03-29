use crate::{error::Result, Db};
use chrono::{NaiveDateTime, Utc};

/// A record in `tbl_friend_request`.
#[derive(Debug, Clone)]
pub struct FriendRequest {
    /// Id of the user making the request.
    pub from_id_user: i32,
    /// Id of the potential friend.
    pub to_id_user: i32,
    /// Date that the friend request was sent.
    pub date_sent: NaiveDateTime,
}

impl FriendRequest {
    /// Inserts a friend request.
    pub async fn insert(db: &Db, id_user: i32, friend_username: &str) -> Result<()> {
        let date_added = Utc::now().naive_utc();
        sqlx::query_file!(
            "sql/friends/insert.sql",
            id_user,
            friend_username,
            date_added
        )
        .execute(db)
        .await?;

        Ok(())
    }
    /// Deletes a friend request.
    pub async fn delete(db: &Db, id_user: i32, friend_username: &str) -> Result<()> {
        sqlx::query_file!("sql/friends/delete.sql", id_user, friend_username)
            .execute(db)
            .await?;

        Ok(())
    }
}
