use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct DbUser {
    pub id_user: String,
    pub username: String,
    pub email: String,
    pub hashed_pass: String,
}

impl DbUser {
    pub async fn find_id(username: &str, pool: &SqlitePool) -> Option<Uuid> {
        sqlx::query!("SELECT id_user FROM tbl_user WHERE username = ?", username)
            .fetch_one(pool)
            .await
            .ok()
            .and_then(|rec| Uuid::parse_str(&rec.id_user).ok())
    }
    pub async fn find_by_id(id_user: &Uuid, pool: &SqlitePool) -> sqlx::Result<Self> {
        let uuid = id_user.to_string();

        sqlx::query_as!(DbUser, "SELECT * FROM tbl_user WHERE id_user = ?", uuid)
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
    // insert this record, return the id of the inserted record.
    pub async fn insert(
        username: &str,
        email: &str,
        hashed_pass: &str,
        pool: &SqlitePool,
    ) -> Option<Uuid> {
        let uuid = Uuid::new_v4();
        let id_user = uuid.to_string();

        sqlx::query!(
            "INSERT INTO tbl_user (id_user, username, email, hashed_pass) VALUES (?, ?, ?, ?)",
            id_user,
            username,
            email,
            hashed_pass
        )
        .execute(pool)
        .await
        .ok()?;

        Some(uuid)
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
