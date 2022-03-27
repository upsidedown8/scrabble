//! API types for live games.

use crate::auth::Auth;
use scrabble::{
    error::GameError,
    game::{play::Play, PlayerNum},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Messages sent during a live game.
#[derive(Debug, Serialize, Deserialize)]
pub enum GameMessage {
    /// First message sent to the server, authenticates the user.
    Authenticate(Auth),
    /// Player took too long to make a move (sent to all players).
    Timeout(Uuid),
    /// Player joined the game (sent to all players).
    Joined(Uuid, PlayerNum),
    /// Player sends a play to the server.
    RequestPlay(Play),
    /// There was an error making the play (invalid play or not the player's move).
    PlayError(GameError),
    /// A validated play was made (sent to all players).
    Play(Play),
    /// Send a chat message.
    RequestChatMessage(ChatMessage),
    /// A message sent in live chat (sent to all players).
    Chat(Uuid, ChatMessage),
}

/// Messages sent in live chat.
#[derive(Debug, Serialize, Deserialize)]
pub enum ChatMessage {
    /// A string message.
    String(String),
}
