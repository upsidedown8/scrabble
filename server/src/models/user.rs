use crate::{
    auth::Role,
    error::{Error, Result},
    Db,
};
use api::routes::users::UserDetails;
use chrono::{NaiveDateTime, Utc};

/// A record in `tbl_user`.
#[derive(Debug, Clone)]
pub struct User {
    /// Id of the user as a string.
    pub id_user: i32,
    /// The username.
    pub username: String,
    /// The email address.
    pub email: String,
    /// The argon2 salted hash of the password.
    pub hashed_pass: String,
    /// The role of the user (User or Admin).
    pub role: String,
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
        self.role.parse().unwrap()
    }
    /// Gets the user id.
    pub fn id_user(&self) -> i32 {
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
    /// Returns Ok(()) if `username` and `email` are not taken (for any user
    /// whose id is different to `id_user`).
    pub async fn check_username_and_email_free(
        db: &Db,
        username: &str,
        email: &str,
        id_user: i32,
    ) -> Result<()> {
        let count: Option<i64> = sqlx::query_file_scalar!(
            "sql/users/username_or_email_exists.sql",
            username,
            email,
            id_user
        )
        .fetch_one(db)
        .await?;
        match count == Some(0) {
            true => Ok(()),
            false => Err(Error::UsernameOrEmailExists),
        }
    }
    /// Finds a user from the user table by id.
    pub async fn find_by_id(db: &Db, id_user: i32) -> Result<Self> {
        sqlx::query_file_as!(User, "sql/users/find_by_id.sql", id_user)
            .fetch_optional(db)
            .await?
            .ok_or(Error::MissingAccount)
    }
    /// Finds a user from the user table by username.
    pub async fn find_by_username(db: &Db, username: &str) -> Result<Self> {
        sqlx::query_file_as!(User, "sql/users/find_by_username.sql", username)
            .fetch_optional(db)
            .await?
            .ok_or(Error::MissingAccount)
    }
    /// Inserts the record into the database.
    pub async fn insert(
        db: &Db,
        username: &str,
        email: &str,
        hashed_pass: &str,
        role: Role,
        is_private: bool,
    ) -> Result<i32> {
        let role = role.to_string();

        let id_user: i32 = sqlx::query_file_scalar!(
            "sql/users/insert.sql",
            username,
            email,
            hashed_pass,
            role,
            is_private,
        )
        .fetch_one(db)
        .await?;

        Ok(id_user)
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
