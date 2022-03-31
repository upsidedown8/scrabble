use crate::{auth::authenticated_user, db::Db, filters::with, handlers};
use warp::{filters::BoxedFilter, Filter, Reply};

/// Combined filter for the friends route.
pub fn all(db: &Db) -> BoxedFilter<(impl Reply,)> {
    add(db)
        .or(remove(db))
        .or(list(db))
        .or(list_requests(db))
        .boxed()
}

/// Add friend.
fn add(db: &Db) -> BoxedFilter<(impl Reply,)> {
    warp::path!("api" / "friends" / String)
        .and(warp::post())
        .and(with(db))
        .and(authenticated_user())
        .and_then(handlers::friends::add)
        .boxed()
}

/// Remove friend.
fn remove(db: &Db) -> BoxedFilter<(impl Reply,)> {
    warp::path!("api" / "friends" / String)
        .and(warp::delete())
        .and(with(db))
        .and(authenticated_user())
        .and_then(handlers::friends::remove)
        .boxed()
}

/// List friends.
fn list(db: &Db) -> BoxedFilter<(impl Reply,)> {
    warp::path!("api" / "friends")
        .and(warp::get())
        .and(with(db))
        .and(authenticated_user())
        .and_then(handlers::friends::list)
        .boxed()
}

/// List friend requests.
fn list_requests(db: &Db) -> BoxedFilter<(impl Reply,)> {
    warp::path!("api" / "friends" / "requests")
        .and(warp::get())
        .and(with(db))
        .and(authenticated_user())
        .and_then(handlers::friends::list_requests)
        .boxed()
}
