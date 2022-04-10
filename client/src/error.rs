//! Module containing the error types.

use api::error::ErrorResponse;
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
    /// Error from the API.
    ApiError(ErrorResponse),
    /// Unexpected HTTP status code.
    HttpStatus(u16),
}

impl std::error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Reqwasm(err) => {
                log::error!("reqwasm error: {err:?}");
                writeln!(f, "Request failed")
            }
            Error::SerdeJson(err) => {
                log::error!("serde error: {err:?}");
                writeln!(f, "Failed to deserialize response body")
            }
            Error::ApiError(err) => {
                let ErrorResponse { status, msg } = err;

                writeln!(f, "API error ({status}): {msg}")
            }
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
