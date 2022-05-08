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
    Api(ErrorResponse),
    /// Unexpected HTTP status code.
    HttpStatus(u16),
    /// Error from JS code (for opening websocket communication).
    Js(gloo_utils::errors::JsError),
    /// Error from sending or receiving a websocket message.
    WebSocket(reqwasm::websocket::WebSocketError),
}

impl std::error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Reqwasm(err) => {
                log::error!("reqwasm error: {err:?}");
                writeln!(
                    f,
                    "Request failed.\n\
                    Check your internet connection."
                )
            }
            Error::SerdeJson(err) => {
                log::error!("serde error: {err:?}");
                writeln!(
                    f,
                    "Failed to deserialize response body.\n\
                    Try clearing your browser's cache and reloading the page.\n\
                    This may be a server-side issue."
                )
            }
            Error::Api(err) => {
                let ErrorResponse { status, msg } = err;
                log::error!("API error ({status}): {msg}");
                writeln!(f, "Error: {msg}")
            }
            Error::HttpStatus(status) => {
                log::error!("Bad response from server.");
                writeln!(
                    f,
                    "Failed to deserialize error message.\n\
                    Try clearing your browser's cache and reloading the page.\n\
                    This may be a server-side issue.\n\
                    (Status {status})"
                )
            }
            Error::Js(_) => writeln!(f, "WebSocket connection error"),
            Error::WebSocket(_) => writeln!(f, "WebSocket communication error"),
        }
    }
}

impl From<reqwasm::websocket::WebSocketError> for Error {
    fn from(err: reqwasm::websocket::WebSocketError) -> Self {
        Error::WebSocket(err)
    }
}
impl From<gloo_utils::errors::JsError> for Error {
    fn from(err: gloo_utils::errors::JsError) -> Self {
        Self::Js(err)
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
