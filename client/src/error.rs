use std::fmt;

#[derive(Debug)]
pub enum Error {
    Reqwasm(reqwasm::Error),
    SerdeJson(serde_json::Error),
}

impl std::error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Reqwasm(err) => writeln!(f, "{err}"),
            Error::SerdeJson(err) => writeln!(f, "{err}"),
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
