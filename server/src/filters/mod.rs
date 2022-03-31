use crate::{error::Error, fsm::FsmRef, Db, Mailer};
use api::error::ErrorResponse;
use std::convert::Infallible;
use warp::{
    body::BodyDeserializeError, filters::BoxedFilter, hyper::StatusCode, Filter, Rejection, Reply,
};

pub mod friends;
pub mod games;
pub mod leaderboard;
// pub mod live;
pub mod users;

/// Gets a filter for all the routes.
pub fn all(hostname: &str, db: Db, mailer: Mailer, fsm: FsmRef) -> BoxedFilter<(impl Reply,)> {
    let api = friends::all(&db)
        .or(games::all(&db))
        .or(leaderboard::all(&db))
        // .or(live::all(&db, &fsm))
        .or(users::all(&db, &mailer));
    let app = warp::fs::dir("static");
    let index = warp::path!()
        .and(warp::get())
        .and(warp::fs::file("static/index.html"));

    // /api/{...} -> API routes
    // /{...}     -> Static files (JS, WASM, CSS and HTML)
    // /{...}     -> Index page (if no other routes match and request is GET).
    let routes = api.or(app).or(index);

    // Ensure the hostname is as specified in `.env` file.
    warp::host::exact(hostname)
        .and(routes)
        .recover(handle_rejection)
        .boxed()
}

/// Gets a filter that extracts `T`.
pub fn with<T: Clone + Send>(data: &T) -> impl Filter<Extract = (T,), Error = Infallible> + Clone {
    let data = data.clone();
    warp::any().map(move || data.clone())
}

/// Handles rejections (errors where all filters fail).
async fn handle_rejection(rejection: Rejection) -> Result<impl Reply, Infallible> {
    let (status, msg) = if let Some(error) = rejection.find::<Error>() {
        log::info!("rejection: {error:?}");
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
            | Error::Env(_)
            | Error::SocketAddr(_)
            | Error::Sqlx(_)
            | Error::Argon2(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        }
    } else if rejection.is_not_found() {
        log::info!("not found");
        (StatusCode::NOT_FOUND, "Not found")
    } else if let Some(error) = rejection.find::<BodyDeserializeError>() {
        log::info!("body deserialize error");
        (StatusCode::BAD_REQUEST, "Invalid request body")
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
