//! Module for error handling.

/// The library result type.
pub type Result<T> = std::result::Result<T, Error>;

impl warp::reject::Reject for Error {}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Self::Sqlx(err)
    }
}
impl From<argon2::Error> for Error {
    fn from(err: argon2::Error) -> Self {
        Self::Argon2(err)
    }
}
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}
impl From<lettre::error::Error> for Error {
    fn from(err: lettre::error::Error) -> Self {
        Self::Lettre(err)
    }
}
impl From<lettre::transport::smtp::Error> for Error {
    fn from(err: lettre::transport::smtp::Error) -> Self {
        Self::Smtp(err)
    }
}
impl From<lettre::address::AddressError> for Error {
    fn from(err: lettre::address::AddressError) -> Self {
        Self::Address(err)
    }
}
impl From<bincode::Error> for Error {
    fn from(err: bincode::Error) -> Self {
        Self::Bincode(err)
    }
}
impl From<std::env::VarError> for Error {
    fn from(err: std::env::VarError) -> Self {
        Self::Env(err)
    }
}
impl From<std::net::AddrParseError> for Error {
    fn from(err: std::net::AddrParseError) -> Self {
        Self::SocketAddr(err)
    }
}

/// The library error type.
#[derive(Debug)]
pub enum Error {
    /// Error loading env variable.
    Env(std::env::VarError),
    /// Error from sqlx.
    Sqlx(sqlx::Error),
    /// Error from argon2.
    Argon2(argon2::Error),
    /// Io error.
    Io(std::io::Error),
    /// Error parsing socket address.
    SocketAddr(std::net::AddrParseError),
    /// Error from lettre.
    Lettre(lettre::error::Error),
    /// Smtp error.
    Smtp(lettre::transport::smtp::Error),
    /// Error parsing email address.
    Address(lettre::address::AddressError),
    /// Error serializing or deserializing data.
    Bincode(bincode::Error),
    /// Error encoding the JWT.
    JwtEncoding(jsonwebtoken::errors::Error),
    /// Error decoding the JWT.
    JwtDecoding(jsonwebtoken::errors::Error),
    /// Username already exists.
    UsernameExists,
    /// User has insufficient access.
    InsufficientRole,
    /// The request is missing an authorization header.
    MissingAuthHeader,
    /// The request has an invalid auth header.
    InvalidAuthHeader,
    /// The request has an incorrect password.
    IncorrectPassword,
    /// Invalid username provided.
    InvalidUsername,
    /// Password was too weak.
    InvalidPassword,
    /// Email was invalid.
    InvalidEmail,
    /// Cannot send a reset password request until the previous
    /// request times out.
    ResetTimeout,
    /// An incorrect secret or username was provided to the password reset
    /// route.
    IncorrectResetSecret,
}
