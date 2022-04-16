use crate::components::Msg;
use api::routes::live::{LiveError, Player, ServerMsg};
use scrabble::game::{play::Play, tile::Tile, GameOverReason};
use std::collections::HashMap;
use sycamore::prelude::{create_rc_signal, RcSignal};

/// The maximum number of messages that will be stored.
const MESSAGE_LIMIT: usize = 20;

/// An internal message received in the app.
#[derive(Debug)]
pub enum AppMsg {
    /// A message received from the server.
    ServerMsg(ServerMsg),
    /// The WebSocket has disconnected.
    WsDisconnect,
}

/// The state of the app.
#[derive(Clone)]
pub enum AppState {
    /// The websocket is connected, but the user needs to create or join a game.
    Connected(Box<ConnectedState>),
    /// The user is playing a game.
    Playing(Box<PlayingState>),
}

impl Default for AppState {
    fn default() -> Self {
        Self::Connected(Box::new(ConnectedState {
            toast: create_rc_signal(None),
        }))
    }
}

/// The state whilst joining or creating a game.
#[derive(Clone)]
pub struct ConnectedState {
    pub toast: RcSignal<Option<String>>,
}

/// The state of a live game.
#[derive(Clone)]
pub struct PlayingState {
    pub id_game: i32,
    pub id_player: i32,
    pub capacity: usize,

    // -- shared state --
    pub players: Vec<Player>,
    pub tiles: Vec<Option<Tile>>,
    pub rack: Vec<Tile>,
    pub scores: HashMap<Player, usize>,
    pub next: Option<Player>,
    pub letter_bag_len: usize,

    // -- local state --
    pub messages: RcSignal<Vec<Msg>>,
}

impl AppState {
    /// Calculates the next state from the previous state and a message.
    pub fn reduce(&self, msg: AppMsg) -> Self {
        match self {
            AppState::Connected(connected) => self.reduce_connected(connected, msg),
            AppState::Playing(playing) => self.reduce_playing(playing, msg),
        }
    }

    /// Implementation of the reduce function when the `AppState` is connected.
    fn reduce_connected(&self, connected: &ConnectedState, msg: AppMsg) -> Self {
        match msg {
            AppMsg::ServerMsg(ServerMsg::Error(e)) => {
                log::error!("failed to join/create: {e:?}");
                connected.toast.set(Some(String::from(match e {
                    LiveError::ZeroPlayers => "No players added",
                    LiveError::IllegalPlayerCount => "Incorrect number of players specified",
                    LiveError::FailedToJoin => "Failed to join",
                    LiveError::InvalidToken => "Provided token was invalid",
                    _ => "Unexpected message",
                })));
            }
            AppMsg::ServerMsg(ServerMsg::Joined {
                id_game,
                id_player,
                capacity,
                players,
                tiles,
                rack,
                scores,
                next,
            }) => {
                let status = match players.len() < capacity {
                    true => "Waiting for players",
                    false => "Playing",
                };

                return AppState::Playing(Box::new(PlayingState {
                    // -- local state --
                    messages: create_rc_signal(vec![Msg {
                        sender: String::from("server"),
                        content: format!(
                            "Joined! (id_game={id_game}, {status} [{}/{capacity}])",
                            players.len()
                        ),
                    }]),
                    capacity,

                    // -- shared state --
                    id_game,
                    id_player,
                    players,
                    tiles,
                    rack,
                    scores,
                    next,
                    letter_bag_len: 100,
                }));
            }
            msg => log::error!("unexpected message: {msg:?}"),
        }

        self.clone()
    }

    /// Implementation of the reduce function when the `AppState` is playing,
    fn reduce_playing(&self, playing: &PlayingState, msg: AppMsg) -> Self {
        match msg {
            AppMsg::ServerMsg(msg) => match msg {
                ServerMsg::Play {
                    player,
                    prev_tiles,
                    play,
                    letter_bag_len,
                    next,
                    scores,
                } => {
                    self.add_server_msg(format!(
                        "{} has made a play. {}",
                        player.username,
                        match &next {
                            Some(player) => format!("It's {} next!", player.username),
                            None => "Game over!".to_string(),
                        }
                    ));

                    // find the next set of tiles.
                    let mut tiles = prev_tiles;
                    if let Play::Place(tile_positions) = &play {
                        for (pos, tile) in tile_positions {
                            tiles[usize::from(*pos)] = Some(*tile);
                        }
                    }

                    return AppState::Playing(Box::new(PlayingState {
                        letter_bag_len,
                        tiles,
                        next,
                        scores,
                        ..playing.clone()
                    }));
                }
                ServerMsg::UserConnected(player) => {
                    self.add_server_msg(format!("{} has joined", player.username));
                }
                ServerMsg::UserDisconnected(player) => {
                    self.add_server_msg(format!("{} has left", player.username));
                }
                ServerMsg::Timeout(player) => {
                    self.add_server_msg(format!("{} has timed out", player.username));
                }
                ServerMsg::Players(players) => {
                    return AppState::Playing(Box::new(PlayingState {
                        players,
                        ..playing.clone()
                    }))
                }
                ServerMsg::Chat(from, msg) => {
                    log::info!("{from:?} said: {msg}");
                    self.add_msg(Msg {
                        sender: from.username,
                        content: msg,
                    });
                }
                ServerMsg::Rack(rack) => {
                    return AppState::Playing(Box::new(PlayingState {
                        rack,
                        ..playing.clone()
                    }));
                }
                ServerMsg::Error(e) => {
                    log::error!("play error: {e:?}");
                    match e {
                        LiveError::Play(e) => self.add_server_msg(format!("Illegal play: {e}")),
                        LiveError::NotYourTurn => {
                            self.add_server_msg(String::from("It's not your turn!"))
                        }
                        _ => (),
                    }
                }
                ServerMsg::Over(reason) => self.add_server_msg(format!(
                    "Game over: {}.",
                    match reason {
                        GameOverReason::TwoPasses => "A player has passed twice",
                        GameOverReason::EmptyRack => "A player has emptied their rack",
                    }
                )),
                ServerMsg::Starting => self.add_server_msg(format!(
                    "The game is starting. It's {} next.",
                    playing.next.as_ref().unwrap().username
                )),
                msg => log::error!("unexpected message: {msg:?}"),
            },
            AppMsg::WsDisconnect => {
                return AppState::default();
            }
        }

        self.clone()
    }

    /// Adds a message from the server to the chat list.
    fn add_server_msg(&self, content: String) {
        self.add_msg(Msg {
            sender: String::from("server"),
            content,
        })
    }

    /// Adds a message to the chat list.
    fn add_msg(&self, msg: Msg) {
        if let Self::Playing(playing) = self {
            // add the latest message to the list.
            let messages = &mut *playing.messages.modify();
            messages.push(msg);

            // pop from the front of the list if it exceeds the maximum length.
            if messages.len() > MESSAGE_LIMIT {
                messages.remove(0);
            }
        }
    }
}
