use crate::{
    db::Db,
    filters::with,
    fsm::FsmRef,
    handlers::{self, live::Games},
};
use warp::{filters::BoxedFilter, ws::Ws, Filter, Reply};

/// Combined filter for the live route.
pub fn all(db: &Db, fsm: &FsmRef) -> BoxedFilter<(impl Reply,)> {
    let games = &Games::new(db, fsm);

    join(games).or(create(games)).boxed()
}

/// Join a game by id.
fn join(games: &Games) -> BoxedFilter<(impl Reply,)> {
    warp::path!("api" / "live" / i32)
        .and(warp::ws())
        .and(with(games))
        .map(|id_game: i32, ws: Ws, games: Games| {
            ws.on_upgrade(move |ws| handlers::live::join(ws, games, id_game));
            warp::reply()
        })
        .boxed()
}

/// Create a game.
fn create(games: &Games) -> BoxedFilter<(impl Reply,)> {
    warp::path!("api" / "live" / "create")
        .and(warp::ws())
        .and(with(games))
        .map(|ws: Ws, games: Games| {
            ws.on_upgrade(|ws| handlers::live::create(ws, games));
            warp::reply()
        })
        .boxed()
}
