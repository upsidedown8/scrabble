//! Module containing the error types.

use std::fmt;

/// The result type for the client.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// The error variants for the client.
#[derive(Debug)]
pub enum Error {
    /// Error originated from making a http or websocket request.
    Reqwasm(reqwasm::Error),
    /// Error originated from serializing or deserializing
    /// data.
    SerdeJson(serde_json::Error),
    /// 400 Bad request
    BadRequest,
    /// 401 Unauthorized
    Unauthorized,
    /// 403 Forbidden
    Forbidden,
    /// 404 Not found
    NotFound,
    /// 500 Internal server error
    InternalServerError,
    /// Any other http status that is not `200..=299` (ok).
    HttpStatus(u16),
}

impl std::error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Reqwasm(err) => writeln!(f, "{err}"),
            Error::SerdeJson(err) => writeln!(f, "{err}"),
            Error::BadRequest => writeln!(f, "Bad Request"),
            Error::Unauthorized => writeln!(f, "Unauthorised"),
            Error::Forbidden => writeln!(f, "Forbidden"),
            Error::NotFound => writeln!(f, "Not Found"),
            Error::InternalServerError => writeln!(f, "Internal Server Error"),
            Error::HttpStatus(status) => writeln!(f, "HTTP status: {status}"),
        }
    }
}

impl From<reqwasm::Error> for Error {
    fn from(err: reqwasm::Error) -> Self {
        Self::Reqwasm(err)
    }
}
impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::SerdeJson(err)
    }
}
