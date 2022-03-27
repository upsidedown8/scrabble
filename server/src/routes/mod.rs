use crate::{error::Error, models::Db};
use api::error::ErrorResponse;
use scrabble::util::fsm::FastFsm;
use std::{convert::Infallible, sync::Arc};
use warp::{http::StatusCode, Filter, Rejection, Reply};

mod leaderboard;
mod live_games;
mod users;

/// Gets a filter for all routes
pub fn all(
    db: &Db,
    fsm: Arc<FastFsm>,
) -> impl Filter<Extract = (impl Reply,), Error = Infallible> + Clone {
    let routes = users::all(db).or(live_games::all(db, fsm));

    warp::path("api").and(routes).recover(handle_rejection)
}

/// Handles rejections (errors where all filters fail)
async fn handle_rejection(rejection: Rejection) -> Result<impl Reply, Infallible> {
    let (status, msg) = if let Some(error) = rejection.find::<Error>() {
        log::error!("rejection: {error:?}");
        match error {
            Error::InsufficientRole => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            Error::InvalidAuthHeader => (StatusCode::BAD_REQUEST, "Invalid auth header"),
            Error::MissingAuthHeader => (StatusCode::NOT_FOUND, "Missing auth header"),
            Error::JwtDecoding(_) => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            Error::JwtEncoding(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
            Error::Sqlx(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
            Error::Uuid(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
            Error::Argon2(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
            Error::IncorrectPassword => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            Error::UsernameExists => (StatusCode::FORBIDDEN, "Username exists"),
            Error::InvalidUsername => (StatusCode::FORBIDDEN, "Username is invalid"),
            Error::InvalidPassword => (StatusCode::FORBIDDEN, "Password is too weak"),
            Error::InvalidEmail => (StatusCode::FORBIDDEN, "Email is invalid"),
        }
    } else if rejection.is_not_found() {
        log::info!("not found");
        (StatusCode::NOT_FOUND, "Not found")
    } else {
        log::error!("unmatched error: {rejection:?}");
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
