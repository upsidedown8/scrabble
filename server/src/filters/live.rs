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

    connect(games).boxed()
}

/// Connect to the server via websocket.
fn connect(games: &Games) -> BoxedFilter<(impl Reply,)> {
    warp::path!("api" / "live")
        .and(warp::ws())
        .and(with(games))
        .map(|ws: Ws, games: Games| {
            ws.on_upgrade(move |ws| handlers::live::connect(ws, games));
            warp::reply()
        })
        .boxed()
}
