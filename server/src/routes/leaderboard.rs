use crate::{
    auth::{authenticated_user, Jwt},
    error::Error,
    with_db, Db,
};
use api::leaderboard::{LeaderboardResponse, LeaderboardRow};
use serde::{Deserialize, Serialize};
use warp::{Filter, Rejection, Reply};

/// Filters for the leaderboard routes.
pub fn all(db: &Db) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let leaderboard_no_auth_route = warp::any()
        .and(warp::get())
        .and(with_db(db))
        .and(warp::query())
        .and_then(leaderboard);
    let leaderboard_auth_route = warp::path("friends")
        .and(warp::get())
        .and(with_db(db))
        .and(authenticated_user())
        .and_then(friends_leaderboard);

    let routes = leaderboard_no_auth_route.or(leaderboard_auth_route);

    warp::path("leaderboard").and(routes)
}

/// Query parameter for the leaderboard route.
#[derive(Serialize, Deserialize)]
struct LeaderboardQuery {
    limit: usize,
    offset: usize,
}

/// GET /api/leaderboard
async fn leaderboard(db: Db, query: LeaderboardQuery) -> Result<impl Reply, Rejection> {
    let limit = query.limit.clamp(10, 50) as i64;
    let offset = query.offset as i64;

    let rows = sqlx::query_file!("sql/leaderboard/overall.sql", limit, offset)
        .fetch_all(&db)
        .await
        .map_err(Error::Sqlx)?
        .into_iter()
        .map(|row| LeaderboardRow {
            username: row.username,
            avg_score_per_play: row.avg_score_per_play.unwrap_or(0.0),
            avg_word_length: row.avg_word_length.unwrap_or(0.0),
            avg_tiles_per_play: row.avg_tiles_per_play.unwrap_or(0.0),
            longest_word_length: row.longest_word_length.unwrap_or(0) as usize,
            best_word_score: row.best_word_score.unwrap_or(0) as usize,
            avg_score_per_game: row.avg_score_per_game.unwrap_or(0.0),
            avg_score_per_tile: row.avg_score_per_tile.unwrap_or(0.0),
            win_percentage: row.win_percentage.unwrap_or(0.0),
        })
        .collect::<Vec<_>>();

    Ok(warp::reply::json(&LeaderboardResponse { rows }))
}

/// GET /api/leaderboard/friends [+Auth]
async fn friends_leaderboard(db: Db, jwt: Jwt) -> Result<impl Reply, Rejection> {
    let id_user = jwt.id_user();
    let rows = sqlx::query_file!("sql/leaderboard/friends.sql", id_user)
        .fetch_all(&db)
        .await
        .map_err(Error::Sqlx)?
        .into_iter()
        .map(|row| LeaderboardRow {
            username: row.username,
            avg_score_per_play: row.avg_score_per_play.unwrap_or(0.0),
            avg_word_length: row.avg_word_length.unwrap_or(0.0),
            avg_tiles_per_play: row.avg_tiles_per_play.unwrap_or(0.0),
            longest_word_length: row.longest_word_length.unwrap_or(0) as usize,
            best_word_score: row.best_word_score.unwrap_or(0) as usize,
            avg_score_per_game: row.avg_score_per_game.unwrap_or(0.0),
            avg_score_per_tile: row.avg_score_per_tile.unwrap_or(0.0),
            win_percentage: row.win_percentage.unwrap_or(0.0),
        })
        .collect::<Vec<_>>();

    // get a leaderboard only containing scores of friends.
    Ok(warp::reply::json(&LeaderboardResponse { rows }))
}
