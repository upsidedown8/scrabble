use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use scrabble::game::{Game, Player};
use uuid::Uuid;
use warp::ws::Ws;

pub struct LiveGames {
    state: Arc<RwLock<Vec<LiveGame>>>,
}

pub struct LiveGame {
    players: HashMap<Player, PlayerType>,
    game: Game,
    id: Uuid,
}

pub enum PlayerType {
    Person(Uuid, LiveGameConnection),
    Computer(Uuid),
}

pub struct LiveGameConnection {
    ws: Ws,
}
