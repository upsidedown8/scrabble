use crate::{
    db::Db,
    filters::with,
    fsm::FsmHandle,
    handlers::{self, live::rooms::RoomsHandle},
};
use warp::{filters::BoxedFilter, ws::Ws, Filter, Reply};

/// Combined filter for the live route.
pub fn all(db: &Db, fsm: &FsmHandle) -> BoxedFilter<(impl Reply,)> {
    let rooms = &RoomsHandle::default();

    connect(db, fsm, rooms).boxed()
}

/// Connect to the server via websocket.
fn connect(db: &Db, fsm: &FsmHandle, rooms: &RoomsHandle) -> BoxedFilter<(impl Reply,)> {
    warp::path!("api" / "live")
        .and(warp::ws())
        .and(with(db))
        .and(with(fsm))
        .and(with(rooms))
        .map(|ws: Ws, db: Db, fsm: FsmHandle, rooms: RoomsHandle| {
            ws.on_upgrade(move |ws| handlers::live::connect(ws, db, fsm, rooms));
            warp::reply()
        })
        .boxed()
}
