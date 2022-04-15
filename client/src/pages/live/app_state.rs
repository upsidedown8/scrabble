use api::routes::live::{Player, ServerMsg};
use scrabble::game::tile::Tile;

/// An internal message received in the app.
pub enum AppMsg {
    /// A message received from the server.
    ServerMsg(ServerMsg),
    /// The WebSocket has disconnected.
    WsDisconnect,
}

/// The state of the app.
pub enum AppState {
    /// The websocket is connected, but the user needs to create or join a game.
    Connected,
    /// The user is playing a game.
    Playing(PlayingState),
}

/// The state of a live game.
pub struct PlayingState {
    pub id_game: i32,
    pub id_user: i32,

    pub is_over: bool,
    pub to_play: Player,
    pub players: Vec<Player>,

    pub rack: Vec<Tile>,
    pub cells: Vec<Option<Tile>>,
}

impl AppState {
    /// Calculates the next state from the previous state and a message.
    pub fn reduce(&self, msg: AppMsg) -> Self {
        todo!()
    }
}
