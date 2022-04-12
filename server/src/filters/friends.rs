use crate::{auth::authenticated_user, db::Db, filters::with, handlers};
use warp::{filters::BoxedFilter, Filter, Reply};

/// Combined filter for the friends route.
pub fn all(db: &Db) -> BoxedFilter<(impl Reply,)> {
    warp::path("friends")
        .and(
            add(db)
                .or(remove(db))
                .or(list(db))
                .or(list_incoming(db))
                .or(list_outgoing(db)),
        )
        .boxed()
}

/// Add friend.
fn add(db: &Db) -> BoxedFilter<(impl Reply,)> {
    warp::path!(String)
        .and(warp::post())
        .and(with(db))
        .and(authenticated_user())
        .and_then(handlers::friends::add)
        .boxed()
}

/// Remove friend.
fn remove(db: &Db) -> BoxedFilter<(impl Reply,)> {
    warp::path!(String)
        .and(warp::delete())
        .and(with(db))
        .and(authenticated_user())
        .and_then(handlers::friends::remove)
        .boxed()
}

/// List friends.
fn list(db: &Db) -> BoxedFilter<(impl Reply,)> {
    warp::path!()
        .and(warp::get())
        .and(with(db))
        .and(authenticated_user())
        .and_then(handlers::friends::list)
        .boxed()
}

/// List incoming friend requests.
fn list_incoming(db: &Db) -> BoxedFilter<(impl Reply,)> {
    warp::path!("requests" / "incoming")
        .and(warp::get())
        .and(with(db))
        .and(authenticated_user())
        .and_then(handlers::friends::list_incoming)
        .boxed()
}

/// List outgoing friend requests.
fn list_outgoing(db: &Db) -> BoxedFilter<(impl Reply,)> {
    warp::path!("requests" / "outgoing")
        .and(warp::get())
        .and(with(db))
        .and(authenticated_user())
        .and_then(handlers::friends::list_outgoing)
        .boxed()
}
