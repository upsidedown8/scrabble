//! API types for live games.

use std::collections::HashMap;

use crate::auth::Token;
use scrabble::{
    error::GameError,
    game::{play::Play, tile::Tile},
};
use serde::{Deserialize, Serialize};

/// Messages sent from the client.
#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMsg {
    /// Request to disconnect.
    Disconnect,
    /// Request to create a game.
    Create {
        /// Number of AI players.
        ai_count: usize,
        /// Number of human players.
        player_count: usize,
        /// Whether the game is closed to friends of the user
        /// that starts the game.
        friends_only: bool,
    },
    /// Request to join a game.
    Join(i32),
    /// A chat message.
    Chat(String),
    /// A play message.
    Play(Play),
    /// The first message sent, authenticates the user.
    Auth(Token),
}

/// Messages sent from the server.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ServerMsg {
    /// A play has been made.
    Play {
        /// The player that made the play.
        player: Player,
        /// The previous board tiles.
        prev_tiles: Vec<Option<Tile>>,
        /// The play that was made.
        play: Play,
        /// The number of tiles remaining in the bag.
        letter_bag_len: usize,
        /// The next player. (None if the game is over).
        next: Option<Player>,
        /// The current scores.
        scores: HashMap<Player, usize>,
    },
    /// The user has joined a game.
    Joined {
        /// Id of the game.
        id_game: i32,
        /// Id of the player that joined.
        id_player: i32,
        /// The required number of players.
        capacity: usize,
        /// The players in the game.
        players: Vec<Player>,
        /// The tile positions.
        tiles: Vec<Option<Tile>>,
        /// Your rack tiles.
        rack: Vec<Tile>,
        /// The current scores.
        scores: HashMap<Player, usize>,
        /// The next player (None if the game is over).
        next: Option<Player>,
    },
    /// A user has connected to the game.
    UserConnected(Player),
    /// A user has disconnected from the game.
    UserDisconnected(Player),
    /// The player has timed out so will disconnect.
    Timeout(Player),
    /// The players have updated.
    Players(Vec<Player>),
    /// A chat message.
    Chat(Player, String),
    /// The player's rack has updated.
    Rack(Vec<Tile>),
    /// An error occured.
    Error(LiveError),
}

/// A member of a game.
#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct Player {
    /// The Id of the player.
    pub id_player: i32,
    /// The username (or AI difficulty) of the player.
    pub username: String,
}

/// Error from the server.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LiveError {
    /// An illegal play was submitted.
    Play(GameError),
    /// Not your turn.
    NotYourTurn,
    /// Cannot create a game containing only Ai players.
    ZeroPlayers,
    /// Must be between 2 and 4 players per game.
    IllegalPlayerCount,
    /// Failed to join a game.
    FailedToJoin,
}
