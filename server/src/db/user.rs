use sqlx::SqlitePool;

#[derive(Debug, Clone)]
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
    pub async fn insert(
        username: &str,
        email: &str,
        hashed_pass: &str,
        pool: &SqlitePool,
    ) -> sqlx::Result<()> {
        sqlx::query!(
            "INSERT INTO tbl_user (username, email, hashed_pass) VALUES (?, ?, ?)",
            username,
            email,
            hashed_pass
        )
        .execute(pool)
        .await
        .map(|_| ())
    }
    pub async fn update(&self, pool: &SqlitePool) -> sqlx::Result<()> {
        sqlx::query!(
            "
            UPDATE tbl_user
            SET
                username = ?,
                email = ?,
                hashed_pass = ?
            WHERE id_user = ?",
            self.username,
            self.email,
            self.hashed_pass,
            self.id_user,
        )
        .execute(pool)
        .await
        .map(|_| ())
    }
    pub async fn delete(&self, pool: &SqlitePool) -> sqlx::Result<()> {
        sqlx::query!("DELETE FROM tbl_user WHERE id_user = ?", self.id_user)
            .execute(pool)
            .await
            .map(|_| ())
    }
}
