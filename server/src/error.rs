pub type Result<T> = std::result::Result<T, Error>;

impl warp::reject::Reject for Error {}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Self::Sqlx(err)
    }
}
impl From<uuid::Error> for Error {
    fn from(err: uuid::Error) -> Self {
        Self::Uuid(err)
    }
}
impl From<argon2::Error> for Error {
    fn from(err: argon2::Error) -> Self {
        Self::Argon2(err)
    }
}

#[derive(Debug)]
pub enum Error {
    Sqlx(sqlx::Error),
    Uuid(uuid::Error),
    Argon2(argon2::Error),
    UsernameExists,
    InsufficientRole,
    MissingAuthHeader,
    InvalidAuthHeader,
    IncorrectPassword,
    JwtEncoding(jsonwebtoken::errors::Error),
    JwtDecoding(jsonwebtoken::errors::Error),
    InvalidUsername,
    InvalidPassword,
    InvalidEmail,
}
