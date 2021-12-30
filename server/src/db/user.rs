use sqlx::SqlitePool;

#[derive(Debug)]
pub struct DbUser {
    pub id_user: i64,
    pub username: String,
    pub email: String,
    pub hashed_pass: String,
}

impl DbUser {
    pub async fn find_by_id(id_user: i64, pool: &SqlitePool) -> sqlx::Result<Self> {
        sqlx::query_as!(DbUser, "SELECT * FROM tbl_user WHERE id_user = ?", id_user)
            .fetch_one(pool)
            .await
    }
    pub async fn find_by_username(username: &str, pool: &SqlitePool) -> sqlx::Result<Self> {
        sqlx::query_as!(
            DbUser,
            "SELECT * FROM tbl_user WHERE username = ?",
            username
        )
        .fetch_one(pool)
        .await
    }
}
