use crate::{auth::Jwt, db::Db, error::Error};
use api::{
    auth::AuthWrapper,
    routes::{
        games::{GameMetadata, GameStatsResponse, ListGamesResponse, OverallStatsResponse},
        leaderboard::LeaderboardRow,
    },
};
use warp::{Rejection, Reply};

/// GET /api/games [+Auth]
pub async fn list(db: Db, jwt: Jwt) -> Result<impl Reply, Rejection> {
    let games = sqlx::query_file!("sql/games/list.sql", jwt.id_user())
        .fetch_all(&db)
        .await
        .map_err(Error::Sqlx)?
        .into_iter()
        .map(|row| GameMetadata {
            id_game: row.id_game,
            start_time: row.start_time,
            end_time: row.end_time,
            is_over: row.is_over,
        })
        .collect();

    Ok(warp::reply::json(&AuthWrapper {
        auth: Some(jwt.auth()?),
        response: ListGamesResponse { games },
    }))
}

/// GET /api/games/{game id}/stats [+Auth]
pub async fn stats(id_game: i32, db: Db, jwt: Jwt) -> Result<impl Reply, Rejection> {
    let row = sqlx::query_file!("sql/games/game_stats.sql", jwt.id_user(), id_game)
        .fetch_one(&db)
        .await
        .map_err(Error::Sqlx)?;
    let game_stats = GameStatsResponse {
        meta: GameMetadata {
            id_game: row.id_game,
            start_time: row.start_time,
            end_time: row.end_time,
            is_over: row.is_over,
        },
        avg_score_per_play: row.avg_score_per_play.unwrap_or(0.0),
        avg_word_length: row.avg_word_length.unwrap_or(0.0),
        avg_words_per_play: row.avg_words_per_play.unwrap_or(0.0),
        avg_tiles_per_play: row.avg_tiles_per_play.unwrap_or(0.0),
        longest_word_length: row.longest_word_length.unwrap_or(0) as usize,
        best_word_score: row.best_word_score.unwrap_or(0) as usize,
        avg_score_per_tile: row.avg_score_per_tile.unwrap_or(0.0),
        is_win: row.is_win.unwrap_or(false),
    };

    Ok(warp::reply::json(&AuthWrapper {
        auth: Some(jwt.auth()?),
        response: game_stats,
    }))
}

/// GET /api/games/stats [+Auth]
pub async fn overall_stats(db: Db, jwt: Jwt) -> Result<impl Reply, Rejection> {
    // Just query a single leaderboard row for this user.
    let row = sqlx::query_file!("sql/games/leaderboard_row.sql", jwt.id_user())
        .fetch_optional(&db)
        .await
        .map_err(Error::Sqlx)?;
    let row = row
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
        .unwrap_or_default();

    Ok(warp::reply::json(&AuthWrapper {
        auth: Some(jwt.auth()?),
        response: OverallStatsResponse { row },
    }))
}
