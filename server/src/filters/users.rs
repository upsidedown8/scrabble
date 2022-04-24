use crate::{auth::authenticated_user, db::Db, filters::with, handlers, mailer::Mailer};
use warp::{filters::BoxedFilter, Filter, Reply};

/// Combined filter for the users route.
pub fn all(db: &Db, mailer: &Mailer) -> BoxedFilter<(impl Reply,)> {
    warp::path("users")
        .and(
            log_in(db)
                .or(sign_up(db))
                .or(profile(db))
                .or(delete(db))
                .or(update(db))
                .or(reset_password(db, mailer))
                .or(reset_with_secret(db)),
        )
        .boxed()
}

/// Log in to an account.
fn log_in(db: &Db) -> BoxedFilter<(impl Reply,)> {
    warp::path!("login")
        .and(warp::post())
        .and(with(db))
        .and(warp::body::json())
        .and_then(handlers::users::log_in)
        .boxed()
}

/// Sign up for an account.
fn sign_up(db: &Db) -> BoxedFilter<(impl Reply,)> {
    warp::path!()
        .and(warp::post())
        .and(with(db))
        .and(warp::body::json())
        .and_then(handlers::users::sign_up)
        .boxed()
}

/// Get account information.
fn profile(db: &Db) -> BoxedFilter<(impl Reply,)> {
    warp::path!()
        .and(warp::get())
        .and(with(db))
        .and(authenticated_user())
        .and_then(handlers::users::profile)
        .boxed()
}

/// Delete an account.
fn delete(db: &Db) -> BoxedFilter<(impl Reply,)> {
    warp::path!()
        .and(warp::delete())
        .and(with(db))
        .and(authenticated_user())
        .and(warp::body::json())
        .and_then(handlers::users::delete)
        .boxed()
}

/// Update an account.
fn update(db: &Db) -> BoxedFilter<(impl Reply,)> {
    warp::path!()
        .and(warp::put())
        .and(with(db))
        .and(authenticated_user())
        .and(warp::body::json())
        .and_then(handlers::users::update)
        .boxed()
}

/// Reset password (sends an email).
fn reset_password(db: &Db, mailer: &Mailer) -> BoxedFilter<(impl Reply,)> {
    warp::path!("reset-password")
        .and(warp::post())
        .and(with(db))
        .and(with(mailer))
        .and(warp::body::json())
        .and_then(handlers::users::reset_password)
        .boxed()
}

/// Reset password from secret sent in email link.
fn reset_with_secret(db: &Db) -> BoxedFilter<(impl Reply,)> {
    warp::path!("reset-password")
        .and(warp::put())
        .and(with(db))
        .and(warp::body::json())
        .and_then(handlers::users::reset_with_secret)
        .boxed()
}
