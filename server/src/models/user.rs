use crate::{
    auth::Role,
    error::{Error, Result},
    Db,
};
use api::users::UserDetails;
use chrono::NaiveDateTime;
use uuid::Uuid;

/// A record in `tbl_user`.
#[derive(Debug, Clone)]
pub struct User {
    /// Uuid of the user as a string.
    pub id_user: String,
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
        Role::parse(&self.role)
    }
    /// Parses the id_user column.
    pub fn id_user(&self) -> Result<Uuid> {
        Uuid::parse_str(&self.id_user).map_err(Error::Uuid)
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
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tbl_user WHERE username = $1")
            .bind(username)
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
            sqlx::query_as!(User, "SELECT * FROM tbl_user WHERE id_user = ?", id_user)
                .fetch_one(db)
                .await?,
        )
    }
    /// Finds a user from the user table by username.
    pub async fn find_by_username(db: &Db, username: &str) -> Result<Self> {
        Ok(
            sqlx::query_as!(User, "SELECT * FROM tbl_user WHERE username = ?", username)
                .fetch_one(db)
                .await?,
        )
    }
    /// Finds a user by email.
    pub async fn find_by_email(db: &Db, email: &str) -> Result<Self> {
        Ok(
            sqlx::query_as!(User, "SELECT * FROM tbl_user WHERE email = ?", email)
                .fetch_one(db)
                .await?,
        )
    }
    /// Inserts the record into the database.
    pub async fn insert(&self, db: &Db) -> Result<()> {
        sqlx::query!(
            "INSERT INTO tbl_user VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
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
        sqlx::query!("DELETE FROM tbl_user WHERE id_user = ?", self.id_user,)
            .execute(db)
            .await?;

        Ok(())
    }
    /// Updates the (email, hashed_pass, username) fields
    /// of the user record (keeping the same id).
    pub async fn update(&self, db: &Db) -> Result<()> {
        sqlx::query!(
            "
            UPDATE tbl_user
            SET username = ?,
                email = ?,
                hashed_pass = ?
            WHERE
                id_user = ?
            ",
            self.username,
            self.email,
            self.hashed_pass,
            self.id_user
        )
        .execute(db)
        .await?;

        Ok(())
    }
}
