//! API types for live games.

use std::collections::HashMap;

use crate::auth::Auth;
use scrabble::game::{play::Play, PlayerNum};
use serde::{Deserialize, Serialize};

/// Info about a player.
#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    /// Id of the player.
    pub id_player: i32,
    /// Username of the player.
    pub username: String,
}

/// The first message sent, authenticates the user.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthMsg(Auth);

/// Messages sent from the client.
#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMsg {
    /// A chat message.
    Chat(String),
    /// Attempt to make a play.
    Play(Play),
}

/// Messages sent from the server.
#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMsg {
    /// A player has left or joined the game.
    Players(HashMap<PlayerNum, Player>),
    /// A chat message.
    Chat(Player, String),
    /// A player has made a play.
    Play(Play),
    /// Sent to a user that sent an illegal play.
    IllegalPlay,
    /// The server state has updated.
    State(ServerState),
}

/// The state of the live game.
#[derive(Debug, Serialize, Deserialize)]
pub enum ServerState {
    /// The server is waiting for players.
    Joining { needed: usize },
    /// It is a player's turn.
    ToPlay(Player),
    /// The game is over.
    Over,
}
