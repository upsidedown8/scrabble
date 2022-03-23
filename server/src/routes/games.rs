use crate::{
    auth::{self, authenticated_user, Jwt},
    models::{with_db, Db},
};
use futures::{FutureExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, io::BufRead};
use tokio::sync::mpsc;
use uuid::Uuid;
use warp::{
    ws::{Message, WebSocket, Ws},
    Filter, Rejection, Reply,
};

/// The filter for the game route.
pub fn all(db: Db) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let create_game_route = warp::any()
        .and(warp::post())
        .and(with_db(db.clone()))
        .and(authenticated_user())
        .and(warp::body::json())
        .and_then(create_game);
    let delete_game_route = warp::any()
        .and(warp::delete())
        .and(with_db(db.clone()))
        .and(authenticated_user())
        .and(warp::path::param())
        .and_then(delete_game);
    let get_plays_route = warp::any()
        .and(warp::get())
        .and(with_db(db.clone()))
        .and(authenticated_user())
        .and(warp::path::param())
        .and(warp::query::query())
        .and_then(get_plays);
    let join_game_route = warp::path("join")
        .and(with_db(db))
        .and(warp::ws())
        .and_then(join_game);

    let routes = join_game_route
        .or(get_plays_route)
        .or(delete_game_route)
        .or(create_game_route);

    warp::path("games").and(routes)
}

/// /api/games/join [websocket]
async fn join_game(db: Db, ws: Ws) -> Result<impl Reply, Infallible> {
    log::info!("ws_echo handler");

    Ok(ws.on_upgrade(on_upgrade))
}
/// Called when the websocket connection succeeds.
async fn on_upgrade(ws: WebSocket) {
    log::info!("establishing client connection... {:#?}", ws);

    let (tx, rx) = ws.split();

    rx.forward(tx)
        .map(|result| {
            if let Err(e) = result {
                eprintln!("ws error: {:?}", e);
            }
        })
        .await;
}

/// POST /api/games [+Auth]
async fn create_game(db: Db, jwt: Jwt, create_game: CreateGame) -> Result<impl Reply, Rejection> {
    todo!()
}

/// DELETE /api/games/:game_id [+Auth]
async fn delete_game(db: Db, jwt: Jwt, game_id: Uuid) -> Result<impl Reply, Rejection> {
    todo!()
}

#[derive(Deserialize)]
struct GetPlaysQuery {
    count: Option<usize>,
    offset: Option<usize>,
}

/// GET /api/games/:game_id/plays&count=0&offset=0
async fn get_plays(
    db: Db,
    jwt: Jwt,
    game_id: Uuid,
    query: GetPlaysQuery,
) -> Result<impl Reply, Rejection> {
    todo!()
}

/// GET /api/games/:game_id/players [+Auth]
async fn get_players(db: Db, jwt: Jwt, game_id: Uuid) -> Result<impl Reply, Rejection> {]
    todo!()
}
