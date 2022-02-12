use crate::{error::Error, models::Db};
use api::error::ErrorResponse;
use std::convert::Infallible;
use warp::{http::StatusCode, Filter, Rejection, Reply};

mod games;
mod leaderboard;
mod users;
mod words;

/// Gets a filter for all routes
pub fn all(db: Db) -> impl Filter<Extract = (impl Reply,), Error = Infallible> + Clone {
    let routes = users::all(db.clone());

    warp::path("api").and(routes).recover(handle_rejection)
}

/// Handles rejections (errors where all filters fail)
async fn handle_rejection(rejection: Rejection) -> Result<impl Reply, Infallible> {
    let (status, msg) = if let Some(error) = rejection.find::<Error>() {
        match error {
            Error::InsufficientRoleError => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            Error::InvalidAuthHeaderError => (StatusCode::BAD_REQUEST, "Invalid auth header"),
            Error::MissingAuthHeaderError => (StatusCode::NOT_FOUND, "Missing auth header"),
            Error::JwtDecodingError(_) => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            Error::JwtEncodingError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            Error::SqlxError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
            Error::UuidError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
            Error::Argon2Error(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
            Error::IncorrectPasswordError => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            Error::UsernameExistsError => (StatusCode::BAD_REQUEST, "Username exists"),
        }
    } else if rejection.is_not_found() {
        (StatusCode::NOT_FOUND, "Not found")
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    };

    Ok(warp::reply::with_status(
        warp::reply::json(&ErrorResponse {
            status: status.to_string(),
            msg: msg.to_string(),
        }),
        status,
    ))
}
