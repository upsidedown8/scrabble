//! API types for live and past games.

use uuid::Uuid;
use scrabble::game::{
    play::Play,
    PlayerId,
};

/// Messages sent during a live game.
pub enum GameMessage {
    /// Player took too long to make a move (sent to all players).
    Timeout(PlayerId),
    /// Player joined the game (sent to all players).
    Joined(PlayerId),
    /// Player sends a play to the server.
    RequestPlay(Play),
    /// There was an error making the play (invalid play or not the player's move).
    PlayError(GameError),
    /// A validated play was made (sent to all players). 
    Play(Play),
    /// Send a chat message.
    SendChatMessage(String),
    /// A message sent in live chat (sent to all players).
    Chat(PlayerId, String),
}

/// Messages sent in live chat.
pub enum ChatMessage {
    /// A string message.
    String(String),
}

/// Body of the request to create a game.
pub struct CreateGame {
    pub player_count: usize,
    pub ai_count: usize,
    pub timeout: Duration,
}

/// Response from creating a game.
pub struct CreateGameResponse {
    pub id_game: usize,
}
