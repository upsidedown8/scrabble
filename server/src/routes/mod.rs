//! Module containing the API endpoints.

use crate::{error::Error, Db, Mailer};
use api::error::ErrorResponse;
use scrabble::util::fsm::FastFsm;
use std::{convert::Infallible, sync::Arc};
use warp::{http::StatusCode, Filter, Rejection, Reply};

mod friends;
mod games;
mod leaderboard;
mod live_games;
mod users;

/// Gets a filter for all routes
pub fn all(
    db: &Db,
    mailer: &Mailer,
    fsm: Arc<FastFsm>,
) -> impl Filter<Extract = (impl Reply,), Error = Infallible> + Clone {
    let api_route = warp::path("api").and(
        users::all(db, mailer)
            .or(live_games::all(db, fsm))
            .or(leaderboard::all(db))
            .or(friends::all(db))
            .or(games::all(db)),
    );
    let static_route = warp::fs::dir("static");
    let index_route = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("static/index.html"));

    // /api/{...} -> API routes
    // /{...}     -> Static files (JS, WASM, CSS and HTML)
    // /{...}     -> Index page (if no other routes match and request is GET).
    api_route
        .or(static_route)
        .or(index_route)
        .recover(handle_rejection)
}

/// Handles rejections (errors where all filters fail)
async fn handle_rejection(rejection: Rejection) -> Result<impl Reply, Infallible> {
    let (status, msg) = if let Some(error) = rejection.find::<Error>() {
        log::error!("rejection: {error:?}");
        match error {
            Error::InvalidAuthHeader => (StatusCode::BAD_REQUEST, "Invalid auth header"),
            Error::MissingAuthHeader => (StatusCode::NOT_FOUND, "Missing auth header"),
            Error::UsernameExists => (StatusCode::FORBIDDEN, "Username exists"),
            Error::InvalidUsername => (StatusCode::FORBIDDEN, "Username is invalid"),
            Error::InvalidPassword => (StatusCode::FORBIDDEN, "Password is too weak"),
            Error::InvalidEmail => (StatusCode::FORBIDDEN, "Email is invalid"),
            Error::ResetTimeout => (
                StatusCode::FORBIDDEN,
                "A recent request was made to reset the password",
            ),
            Error::JwtDecoding(_)
            | Error::IncorrectResetSecret
            | Error::IncorrectPassword
            | Error::InsufficientRole => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            Error::Lettre(_)
            | Error::Address(_)
            | Error::JwtEncoding(_)
            | Error::Bincode(_)
            | Error::Io(_)
            | Error::Smtp(_)
            | Error::Sqlx(_)
            | Error::Argon2(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
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
