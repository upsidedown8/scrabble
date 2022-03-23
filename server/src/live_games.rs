use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use scrabble::game::{Game, PlayerId};
use uuid::Uuid;
use warp::ws::Ws;

pub struct LiveGames {
    state: Arc<RwLock<Vec<LiveGame>>>,
}

pub struct LiveGame {
    id_game: Uuid,
    players: HashMap<PlayerId, PlayerType>,
    game: Game,
}

pub enum PlayerType {
    Person(Uuid, LiveGameConnection),
    Computer(Uuid),
}

pub struct LiveGameConnection {
    ws: Ws,
}
