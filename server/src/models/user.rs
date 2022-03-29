use crate::{
    auth::Role,
    error::{Error, Result},
    Db,
};
use api::users::UserDetails;
use chrono::{NaiveDateTime, Utc};

/// A record in `tbl_user`.
#[derive(Debug, Clone)]
pub struct User {
    /// Id of the user as a string.
    pub id_user: usize,
    /// The username.
    pub username: String,
    /// The email address.
    pub email: String,
    /// The argon2 salted hash of the password.
    pub hashed_pass: String,
    /// The role of the user (User or Admin).
    pub role: Role,
    /// Whether the user stats are private.
    pub is_private: bool,
    /// The date that the user created their account.
    pub date_joined: NaiveDateTime,
    /// The most recent update to the user's account.
    pub date_updated: NaiveDateTime,
}

impl User {
    /// Parses the role column.
    pub fn role(&self) -> Role {
        self.role
    }
    /// Gets the user id.
    pub fn id_user(&self) -> usize {
        self.id_user
    }
    /// Converts to `api::users::UserDetails`.
    pub fn into_user_details(self) -> UserDetails {
        UserDetails {
            username: self.username,
            email: self.email,
            is_private: self.is_private,
        }
    }
    /// Returns Ok(()) if `username` is not taken.
    pub async fn check_username_free(db: &Db, username: &str) -> Result<()> {
        let count: i32 = sqlx::query_file_scalar!("sql/users/count_username.sql", username)
            .fetch_one(db)
            .await?;
        match count == 0 {
            true => Ok(()),
            false => Err(Error::UsernameExists),
        }
    }
    /// Finds a user from the user table by id.
    pub async fn find_by_id(db: &Db, id_user: &Uuid) -> Result<Self> {
        let id_user = id_user.to_string();

        Ok(
            sqlx::query_file_as!(User, "sql/users/find_by_id.sql", id_user)
                .fetch_one(db)
                .await?,
        )
    }
    /// Finds a user from the user table by username.
    pub async fn find_by_username(db: &Db, username: &str) -> Result<Self> {
        Ok(
            sqlx::query_file_as!(User, "sql/users/find_by_username.sql", username)
                .fetch_one(db)
                .await?,
        )
    }
    /// Finds a user by email.
    pub async fn find_by_email(db: &Db, email: &str) -> Result<Self> {
        Ok(
            sqlx::query_file_as!(User, "sql/users/find_by_email.sql", email)
                .fetch_one(db)
                .await?,
        )
    }
    /// Inserts the record into the database.
    pub async fn insert(&self, db: &Db) -> Result<()> {
        sqlx::query_file!(
            "sql/users/insert.sql",
            self.id_user,
            self.username,
            self.email,
            self.hashed_pass,
            self.role,
            self.is_private,
            self.date_joined,
            self.date_updated
        )
        .execute(db)
        .await?;

        Ok(())
    }
    /// Deletes the record by id.
    pub async fn delete(&self, db: &Db) -> Result<()> {
        sqlx::query_file!("sql/users/delete.sql", self.id_user,)
            .execute(db)
            .await?;

        Ok(())
    }
    /// Updates the (email, hashed_pass, username, date_updated, is_private) fields
    /// of the user record (keeping the same id).
    pub async fn update(&self, db: &Db) -> Result<()> {
        let date_updated = Utc::now().naive_utc();

        sqlx::query_file!(
            "sql/users/update.sql",
            self.username,
            self.email,
            self.hashed_pass,
            self.is_private,
            date_updated,
            self.id_user,
        )
        .execute(db)
        .await?;

        Ok(())
    }
}
