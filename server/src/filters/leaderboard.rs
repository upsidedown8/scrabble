use crate::{auth::authenticated_user, db::Db, filters::with, handlers};
use serde::{Deserialize, Serialize};
use warp::{filters::BoxedFilter, Filter, Reply};

/// Combined filter for the leaderboard route.
pub fn all(db: &Db) -> BoxedFilter<(impl Reply,)> {
    overall_leaderboard(db).or(friends_leaderboard(db)).boxed()
}

/// Query parameter for the leaderboard route.
#[derive(Serialize, Deserialize)]
pub struct LeaderboardQuery {
    pub limit: usize,
    pub offset: usize,
}

/// The overall leaderboard.
fn overall_leaderboard(db: &Db) -> BoxedFilter<(impl Reply,)> {
    warp::path!("api" / "leaderboard")
        .and(warp::get())
        .and(with(db))
        .and(warp::query())
        .and_then(handlers::leaderboard::overall_leaderboard)
        .boxed()
}

/// A leaderboard that only contains the user's friends.
fn friends_leaderboard(db: &Db) -> BoxedFilter<(impl Reply,)> {
    warp::path!("api" / "leaderboard" / "friends")
        .and(warp::get())
        .and(with(db))
        .and(authenticated_user())
        .and_then(handlers::leaderboard::friends_leaderboard)
        .boxed()
}
