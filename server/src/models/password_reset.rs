use crate::{error::Result, Db};
use chrono::{NaiveDateTime, Utc};
use uuid::Uuid;

/// A record in `tbl_password_reset`.
#[derive(Debug)]
pub struct PasswordReset {
    /// The id of the user who made the request.
    pub id_user: String,
    /// A secret which is sent by email to verify the user's
    /// password reset request.
    pub secret_hex: String,
    /// The time at which the `password_reset` is no longer valid.
    pub valid_until: NaiveDateTime,
}

impl PasswordReset {
    /// Checks whether the password reset record has expired.
    pub fn is_expired(&self) -> bool {
        let current_time = Utc::now().naive_utc();
        self.valid_until < current_time
    }
    /// Finds a `PasswordReset` record by user id.
    pub async fn find_by_id_user(db: &Db, id_user: &Uuid) -> Result<Self> {
        let id_user = id_user.to_string();

        Ok(sqlx::query_file_as!(
            PasswordReset,
            "sql/password_reset/find_by_id_user.sql",
            id_user
        )
        .fetch_one(db)
        .await?)
    }
    /// Finds a `PasswordReset` record by username.
    pub async fn find_by_username(db: &Db, username: &str) -> Result<Self> {
        Ok(sqlx::query_file_as!(
            PasswordReset,
            "sql/password_reset/find_by_username.sql",
            username
        )
        .fetch_one(db)
        .await?)
    }
    /// Inserts the record into the database.
    pub async fn insert(&self, db: &Db) -> Result<()> {
        sqlx::query_file!(
            "sql/password_reset/insert.sql",
            self.id_user,
            self.secret_hex,
            self.valid_until,
        )
        .execute(db)
        .await?;

        Ok(())
    }
    /// Deletes the record.
    pub async fn delete(&self, db: &Db) -> Result<()> {
        sqlx::query_file!("sql/password_reset/delete.sql", self.id_user,)
            .execute(db)
            .await?;

        Ok(())
    }
    /// Check that the secret matches another value.
    pub fn secret_matches(&self, hex: &str) -> bool {
        self.secret_hex == hex
    }
}
