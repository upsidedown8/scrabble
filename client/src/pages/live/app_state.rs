use crate::components::Msg;
use api::routes::live::{LiveError, Player, ServerMsg};
use scrabble::game::{play::Play, tile::Tile, GameOverReason};
use std::collections::HashMap;
use sycamore::prelude::{create_rc_signal, RcSignal};

/// The maximum number of messages that will be stored.
const MESSAGE_LIMIT: usize = 20;

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
    pub tiles: RcSignal<Vec<Option<Tile>>>,
    pub rack: RcSignal<Vec<Tile>>,
    pub scores: RcSignal<HashMap<Player, usize>>,
    pub next: RcSignal<Option<Player>>,
    pub letter_bag_len: RcSignal<usize>,
    pub is_playing: RcSignal<bool>,

    // -- local state --
    pub messages: RcSignal<Vec<Msg>>,
}

impl AppState {
    /// Calculates the next state from the previous state and a message.
    pub fn reduce(&self, msg: ServerMsg) -> Self {
        match self {
            AppState::Connected(connected) => self.reduce_connected(connected, msg),
            AppState::Playing(playing) => self.reduce_playing(playing, msg),
        }
    }

    /// Implementation of the reduce function when the `AppState` is connected.
    fn reduce_connected(&self, connected: &ConnectedState, msg: ServerMsg) -> Self {
        match msg {
            ServerMsg::Error(e) => {
                log::error!("failed to join/create: {e:?}");
                connected.toast.set(Some(String::from(match e {
                    LiveError::ZeroPlayers => "No players added",
                    LiveError::IllegalPlayerCount => "Incorrect number of players specified",
                    LiveError::FailedToJoin => "Failed to join",
                    LiveError::InvalidToken => "Provided token was invalid",
                    _ => "Unexpected message",
                })));
            }
            ServerMsg::Joined {
                id_game,
                id_player,
                capacity,
                tiles,
                rack,
                scores,
                next,
            } => {
                let status = match scores.len() < capacity {
                    true => "Waiting for players",
                    false => "Playing",
                };

                return AppState::Playing(Box::new(PlayingState {
                    // -- local state --
                    messages: create_rc_signal(vec![Msg {
                        sender: String::from("server"),
                        content: format!(
                            "Joined! (id_game={id_game}, {status} [{}/{capacity}])",
                            scores.len()
                        ),
                    }]),
                    capacity,

                    // -- shared state --
                    id_game,
                    id_player,
                    tiles: create_rc_signal(tiles),
                    rack: create_rc_signal(rack),
                    scores: create_rc_signal(scores),
                    next: create_rc_signal(next),
                    letter_bag_len: create_rc_signal(100),
                    is_playing: create_rc_signal(false),
                }));
            }
            msg => log::error!("unexpected message: {msg:?}"),
        }

        self.clone()
    }

    /// Implementation of the reduce function when the `AppState` is playing,
    fn reduce_playing(&self, playing: &PlayingState, msg: ServerMsg) -> Self {
        match msg {
            ServerMsg::Play {
                player,
                prev_tiles,
                play,
                letter_bag_len,
                next,
                scores,
            } => {
                self.add_server_msg(format!(
                    "{} has made a play ({}). {}",
                    player.username,
                    match play {
                        Play::Pass => "Passed",
                        Play::Redraw(..) => "Redraw tiles",
                        Play::Place(..) => "Placed tiles",
                    },
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

                playing.letter_bag_len.set(letter_bag_len);
                playing.tiles.set(tiles);
                playing.next.set(next);
                playing.scores.set(scores);
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
            ServerMsg::Players(scores) => {
                playing.scores.set(scores);
            }
            ServerMsg::Chat(from, msg) => {
                log::info!("{from:?} said: {msg}");
                self.add_msg(Msg {
                    sender: from.username,
                    content: msg,
                });
            }
            ServerMsg::Rack(rack) => {
                playing.rack.set(rack);
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
            ServerMsg::Over(reason) => {
                playing.is_playing.set(false);
                self.add_server_msg(format!(
                    "Game over: {}.",
                    match reason {
                        GameOverReason::TwoPasses => "A player has passed twice",
                        GameOverReason::EmptyRack => "A player has emptied their rack",
                    }
                ))
            }
            ServerMsg::Starting => {
                playing.is_playing.set(true);

                match playing.next.get().as_ref() {
                    Some(Player { username, .. }) => {
                        self.add_server_msg(format!("The game is starting. It's {username} next.",))
                    }
                    None => log::error!("next player was not specified"),
                }
            }
            msg => log::error!("unexpected message: {msg:?}"),
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
