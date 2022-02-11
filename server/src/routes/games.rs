use crate::auth::{self, AuthenticatedUser};
use crate::db::user::DbUser;

use crate::{routes::Response, AppState};
use api::{users::*, games::*};
use uuid::Uuid;
use rocket::{http::Status, serde::json::Json, State};

#[post("/games", data = "<req>", format = "json")]
pub async fn create_game(
    state: &State<AppState<'_>>,
    req: Json<GameCreate>,
    user: AuthenticatedUser,
) -> Response<GameCreateResponse> {
    todo!()
}

#[delete("/games/<game_id>")]
pub async fn delete_game(
    state: &State<AppState>,
    req: Json<GameDelete>,
    user: AuthenticatedUser,
    game_id: Uuid,
) -> Response<GameDeleteResponse> {
    todo!()
}

#[post("/games/<game_id>/plays", data = "req", format = "json")]
pub async fn make_play(
    state: &State<AppState>,
    req: Json<GameMakePlay>,
    user: AuthenticatedUser,
    game_id: Uuid,
) -> Response<GameMakePlayResponse> {
    todo!()
}

#[get("/games/<game_id>/plays&count=0")]
pub async fn get_plays(
    state: &State<AppState>,
    req: Json<GameGetPlays>,
    user: AuthenticatedUser,
    count: usize,
    game_id: Uuid,
) -> Response<GameGetPlaysResponse> {
    todo!()
}

#[get("/games/<game_id>/players")]
pub async fn get_players(
    state: &State<AppState>,
    req: Json<GameGetPlayers>,
    user: AuthenticatedUser,
    game_id: Uuid,
) -> Response<GameGetPlayersResponse> {
    todo!()
}

#[post("/games/<game_id>/players")]
pub async fn join_game(
    state: &State<AppState>,
    req: Json<GameJoin>,
    user: AuthenticatedUser,
    game_id: Uuid,
) -> Response<GameJoinResponse> {
    todo!()
}

#[delete("/games/<game_id>/players")]
pub async fn join_game(
    state: &State<AppState>,
    req: Json<GameJoin>,
    user: AuthenticatedUser,
    game_id: Uuid,
) -> Response<GameJoinResponse> {
    todo!()
}
