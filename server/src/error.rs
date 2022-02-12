pub type Result<T> = std::result::Result<T, Error>;

impl warp::reject::Reject for Error {}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Self::SqlxError(err)
    }
}
impl From<uuid::Error> for Error {
    fn from(err: uuid::Error) -> Self {
        Self::UuidError(err)
    }
}
impl From<argon2::Error> for Error {
    fn from(err: argon2::Error) -> Self {
        Self::Argon2Error(err)
    }
}

#[derive(Debug)]
pub enum Error {
    SqlxError(sqlx::Error),
    UuidError(uuid::Error),
    Argon2Error(argon2::Error),
    UsernameExistsError,
    InsufficientRoleError,
    MissingAuthHeaderError,
    InvalidAuthHeaderError,
    IncorrectPasswordError,
    JwtEncodingError(jsonwebtoken::errors::Error),
    JwtDecodingError(jsonwebtoken::errors::Error),
}
