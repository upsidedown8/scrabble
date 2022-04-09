use crate::{
    db::Db,
    error::{Error, Result},
    fsm::FsmHandle,
    handlers, Mailer,
};
use api::error::ErrorResponse;
use std::{convert::Infallible, env};
use warp::{
    body::BodyDeserializeError, filters::BoxedFilter, hyper::StatusCode, path::Peek, Filter,
    Rejection, Reply,
};

pub mod friends;
pub mod games;
pub mod leaderboard;
pub mod live;
pub mod users;

/// Gets a filter that servers the API.
fn api_filter(db: Db, mailer: Mailer, fsm: FsmHandle) -> BoxedFilter<(impl Reply,)> {
    warp::path("api")
        .and(
            friends::all(&db)
                .or(games::all(&db))
                .or(leaderboard::all(&db))
                .or(live::all(&db, &fsm))
                .or(users::all(&db, &mailer)),
        )
        .boxed()
}

/// Gets a filter that checks that the route is not the api (used for
/// serving the index page).
fn is_static_route() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    warp::path::peek()
        .and_then(|path: Peek| async move {
            match path.as_str().starts_with("api") {
                true => Err(warp::reject::reject()),
                false => Ok(()),
            }
        })
        .untuple_one()
}

/// Gets a filter that serves the Single Page App.
fn app_filter() -> BoxedFilter<(impl Reply,)> {
    let app = warp::fs::dir("static");
    let index = warp::any()
        .and(warp::get())
        .and(warp::fs::file("static/index.html"));

    warp::any()
        .and(is_static_route())
        .and(app.or(index))
        .boxed()
}

/// A filter that redirects HTTP to HTTPS.
pub fn http_redirect() -> BoxedFilter<(impl Reply,)> {
    warp::any()
        .and(warp::host::optional())
        .and(warp::path::full())
        .and_then(|authority, full_path| async { handlers::http_redirect(authority, full_path) })
        .boxed()
}

/// Gets a filter for all the routes.
pub fn all(db: Db, mailer: Mailer, fsm: FsmHandle) -> Result<BoxedFilter<(impl Reply,)>> {
    let api = api_filter(db, mailer, fsm);
    let app = app_filter();

    let host = env::var("DOMAIN")?;

    Ok(warp::host::exact(&host).and(api.or(app)).boxed())
}

/// Gets a filter that extracts `T`.
pub fn with<T: Clone + Send>(data: &T) -> impl Filter<Extract = (T,), Error = Infallible> + Clone {
    let data = data.clone();
    warp::any().map(move || data.clone())
}

/// Handles rejections (errors where all filters fail).
pub async fn handle_rejection(rejection: Rejection) -> Result<impl Reply, Infallible> {
    let (status, msg) = if let Some(error) = rejection.find::<Error>() {
        log::info!("rejection: {error:?}");
        match error {
            Error::Http(_) => (StatusCode::BAD_REQUEST, "Bad request"),
            Error::MissingAuthHeader => (StatusCode::NOT_FOUND, "Missing auth header"),
            Error::UsernameOrEmailExists => (StatusCode::FORBIDDEN, "Username or email exists"),
            Error::InvalidUsername => (StatusCode::FORBIDDEN, "Username is invalid"),
            Error::InvalidPassword => (StatusCode::FORBIDDEN, "Password is too weak"),
            Error::InvalidEmail => (StatusCode::FORBIDDEN, "Email is invalid"),
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
            Error::InvalidAuthHeader | Error::MissingAuthority => {
                (StatusCode::BAD_REQUEST, "Invalid auth header")
            }
            Error::ResetTimeout => (
                StatusCode::FORBIDDEN,
                "A recent request was made to reset the password",
            ),
            Error::ResetExpired => (
                StatusCode::FORBIDDEN,
                "The request to reset your password has expired",
            ),
            Error::MissingAccount | Error::IncorrectPassword => {
                (StatusCode::UNAUTHORIZED, "Incorrect username or password")
            }
            Error::JwtDecoding(_) | Error::IncorrectResetSecret | Error::InsufficientRole => {
                (StatusCode::UNAUTHORIZED, "Unauthorized")
            }
        }
    } else if rejection.is_not_found() {
        log::info!("not found");
        (StatusCode::NOT_FOUND, "Not found")
    } else if rejection.find::<BodyDeserializeError>().is_some() {
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
