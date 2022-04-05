use crate::{auth::authenticated_user, db::Db, filters::with, handlers};
use warp::{filters::BoxedFilter, Filter, Reply};

/// Combined filter for the games route.
pub fn all(db: &Db) -> BoxedFilter<(impl Reply,)> {
    warp::path("games")
        .and(list(db).or(stats(db)).or(overall_stats(db)))
        .boxed()
}

/// List games that the user has played.
fn list(db: &Db) -> BoxedFilter<(impl Reply,)> {
    warp::path!()
        .and(warp::get())
        .and(with(db))
        .and(authenticated_user())
        .and_then(handlers::games::list)
        .boxed()
}

/// Get stats for a particular game.
fn stats(db: &Db) -> BoxedFilter<(impl Reply,)> {
    warp::path!(i32 / "stats")
        .and(warp::get())
        .and(with(db))
        .and(authenticated_user())
        .and_then(handlers::games::stats)
        .boxed()
}

/// Get stats over all games a user has played.
fn overall_stats(db: &Db) -> BoxedFilter<(impl Reply,)> {
    warp::path!("stats")
        .and(warp::get())
        .and(with(db))
        .and(authenticated_user())
        .and_then(handlers::games::overall_stats)
        .boxed()
}
