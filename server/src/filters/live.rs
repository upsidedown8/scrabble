use crate::{
    db::Db,
    filters::with,
    fsm::FsmHandle,
    handlers::{self, live::games::GamesHandle},
};
use warp::{filters::BoxedFilter, ws::Ws, Filter, Reply};

/// Combined filter for the live route.
pub fn all(db: &Db, fsm: &FsmHandle) -> BoxedFilter<(impl Reply,)> {
    let games = GamesHandle::new(db, fsm);

    warp::path("live").and(connect(&games)).boxed()
}

/// Connect to the server via websocket.
fn connect(games: &GamesHandle) -> BoxedFilter<(impl Reply,)> {
    warp::path!()
        .and(warp::ws())
        .and(with(games))
        .map(|ws: Ws, games: GamesHandle| {
            ws.on_upgrade(move |ws| handlers::live::connected(ws, games));
            warp::reply()
        })
        .boxed()
}
