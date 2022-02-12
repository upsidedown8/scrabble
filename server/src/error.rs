pub type Result<T> = std::result::Result<T, Error>;

impl warp::reject::Reject for Error {}

#[derive(Clone, Debug)]
pub enum Error {
    InsufficientRoleError,
    MissingAuthHeaderError,
    InvalidAuthHeaderError,
    JwtEncodingError(jsonwebtoken::errors::Error),
    JwtDecodingError(jsonwebtoken::errors::Error),
}
