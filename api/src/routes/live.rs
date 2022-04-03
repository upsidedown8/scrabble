//! API types for live games.

use crate::auth::Auth;
use serde::{Deserialize, Serialize};

/// Messages sent from the client.
#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMsg {
    /// The first message sent, authenticates the user.
    Auth(Auth),
    /// Request to join a game.
    Join(i32),
    /// Request to create a game with a fixed number of players.
    Create(usize),
    /// A chat message.
    Chat(String),
}

/// Messages sent from the server.
#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMsg {
    /// A chat message.
    Chat(i32, String),
    /// An error occured.
    Error(Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Error {}
