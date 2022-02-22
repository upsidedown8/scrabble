use std::fmt;

#[derive(Debug)]
pub enum Error {
    Reqwasm(reqwasm::Error),
    SerdeJson(serde_json::Error),
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    InternalServerError,
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