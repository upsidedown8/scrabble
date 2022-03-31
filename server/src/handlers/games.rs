use crate::{auth::Jwt, db::Db, error::Error};
use api::{
    auth::AuthWrapper,
    routes::{
        games::{GameMetadata, GameStatsResponse, ListGamesResponse, OverallStatsResponse},
        leaderboard::LeaderboardRow,
    },
};
use warp::{Rejection, Reply};

/// GET /games [+Auth]
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

/// GET /games/{game id}/stats [+Auth]
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

/// GET /games/stats [+Auth]
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

// /// GET /games/{game id} [+Auth]
// pub async fn get_game(_db: Db, _jwt: Jwt, _id_game: i32) -> Result<impl Reply, Rejection> {
//     // get the game data.
//     let game = models::Game::find_by_id_and_user(&db, jwt.id_user(), id_game).await?;
//     // get the player data.
//     let players = sqlx::query_file!("sql/games/players_by_game_id.sql", id_game)
//         .fetch_all(&db)
//         .await
//         .map_err(Error::Sqlx)?
//         .into_iter()
//         .map(|row| GamePlayer {
//             id_player: row.id_player,
//             username: row.username,
//         })
//         .collect();

//     struct DbPlay {
//         id_play: i32,
//         id_player: i32,
//         letters_removed: String,
//         letters_added: String,
//     }
//     struct DbTile {
//         id_play: i32,
//         pos: i32,
//         ch: char,
//         is_blank: bool,
//     }

//     let mut play_map: HashMap<i32, Vec<(Pos, Tile)>> = HashMap::default();

//     // get the plays from the game.
//     let db_plays: Vec<_> = sqlx::query_file!("sql/games/plays_by_game_id.sql", id_game)
//         .fetch_all(&db)
//         .await
//         .map_err(Error::Sqlx)?
//         .into_iter()
//         .map(|row| DbPlay {
//             id_play: row.id_play,
//             id_player: row.id_player,
//             letters_removed: row.letters_removed,
//             letters_added: row.letters_added,
//         })
//         .collect();

//     for play in db_plays.iter() {
//         play_map.insert(play.id_play, vec![]);
//     }

//     // get the tiles from the game.
//     let db_tiles: Vec<_> = sqlx::query_file!("sql/games/tiles_by_game_id.sql", id_game)
//         .fetch_all(&db)
//         .await
//         .map_err(Error::Sqlx)?
//         .into_iter()
//         .map(|row| DbTile {
//             id_play: row.id_play,
//             pos: row.pos,
//             ch: row.letter.chars().next().expect("a letter"),
//             is_blank: row.is_blank,
//         })
//         .collect();

//     // populate the hashmap with (pos, tile) tuples.
//     for db_tile in db_tiles {
//         if let Some(elem) = play_map.get_mut(&db_tile.id_play) {
//             if let Some(letter) = Letter::new(db_tile.ch) {
//                 let tile = match db_tile.is_blank {
//                     false => Tile::from(letter),
//                     true => Tile::from(Some(letter)),
//                 };
//                 let pos = Pos::from(db_tile.pos as usize);

//                 elem.push((pos, tile));
//             }
//         }
//     }

//     // iterate over the db plays to find the final plays.
//     let plays = db_plays
//         .into_iter()
//         .map(|db_play| match play_map.get(&db_play.id_play) {
//             Some(tile_positions) => todo!(),
//             None => todo!(),
//         });

//     Ok(warp::reply::json(&AuthWrapper {
//         auth: Some(jwt.auth()?),
//         response: GameResponse {
//             meta: GameMetadata {
//                 id_game: game.id_game,
//                 start_time: game.start_time,
//                 end_time: game.end_time,
//                 is_over: game.is_over,
//             },
//             players,
//             plays,
//         },
//     }))

//     Ok(warp::reply())
// }
