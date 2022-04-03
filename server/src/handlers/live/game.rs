use crate::{
    db::Db,
    fsm::FsmHandle,
    handlers::live::player::{GamePlayer, Slot},
    models::AiDifficulty,
};
use api::routes::live::{ClientMsg, LiveError, Player, ServerMsg};
use scrabble::{
    ai::Ai,
    error::GameError,
    game::{play::Play, PlayerNum},
    util::fsm::FastFsm,
};
use std::{env, ops::Deref, sync::Arc, time::Duration};
use tokio::sync::{mpsc, Mutex};

lazy_static::lazy_static! {
    static ref USER_TIMEOUT: Duration = {
        let seconds = env::var("USER_TIMEOUT").expect("`USER_TIMEOUT` env var");
        let seconds = seconds.parse().expect("`USER_TIMEOUT` should be an integer");

        Duration::from_secs(seconds)
    };
}

/// A message sent to a game. (ClientMsg + user id).
#[derive(Debug)]
pub struct GameMsg {
    pub id_user: i32,
    pub msg: ClientMsg,
}
impl GameMsg {
    /// Creates a new [`GameMsg`].
    pub fn new(id_user: i32, msg: ClientMsg) -> GameMsg {
        GameMsg { id_user, msg }
    }
}

/// A thread-safe handle to a particular game.
#[derive(Clone, Debug)]
pub struct GameHandle(Arc<Mutex<Game>>);
impl GameHandle {
    /// Creates a new [`GameHandle`] from the player count, returning
    /// the id of the game.
    pub async fn create(db: Db, fsm: FsmHandle) -> (i32, GameHandle) {
        // // create a queue that allows connected clients to send messages
        // // to the game (multiple producers) and the game to receive the
        // // message (single consumer).
        // let (sender, mut receiver) = mpsc::unbounded_channel::<GameMsg>();

        // let id_game = GAME_ID.fetch_add(1, Ordering::Relaxed);
        // let game = Game {
        //     id_game,
        //     sender,
        //     players: vec![],
        //     db,
        //     fsm,
        //     game: scrabble::game::Game::with_players(4),
        // };
        // let game_handle = GameHandle(Arc::new(Mutex::new(game)));

        // // Spawn a task to listen for messages and act on them.
        // tokio::spawn({
        //     let game_handle = game_handle.clone();
        //     async move {
        //         while let Some(msg) = receiver.recv().await {
        //             log::debug!("game message received: {msg:?}");
        //             // lock the game and handle the message.
        //             // let mut game_lock = game_handle.lock().await;
        //             // game_lock.on_msg(msg).await;
        //         }

        //         log::info!("closing game");
        //     }
        // });

        // (id_game, game_handle)

        todo!()
    }
}
impl Deref for GameHandle {
    type Target = Mutex<Game>;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

/// A game, manages connections and communication between clients,
/// and maintains the game state to mediate plays.
#[derive(Debug)]
pub struct Game {
    /// Id of the game.
    id_game: i32,
    /// Id of the initiator of the game. If set, only friends of the owner
    /// can join.
    id_owner: Option<i32>,
    /// Stores the number of plays. This is used to determine whether
    /// any moves were made since the timeout started.
    plays: usize,

    /// Game state.
    game: scrabble::game::Game,
    /// The fixed slots that players may join.
    slots: Vec<Option<Slot>>,

    /// Database pool.
    db: Db,
    /// Handle to Fsm.
    fsm: FsmHandle,

    /// Clients can use this sender to send
    /// messages to the game.
    sender: mpsc::UnboundedSender<GameMsg>,
}
impl Game {
    /// Gets the game id.
    pub fn id_game(&self) -> i32 {
        self.id_game
    }
    /// Gets the sender for the game,
    pub fn sender(&self) -> mpsc::UnboundedSender<GameMsg> {
        self.sender.clone()
    }

    /// Handles a message from a client.
    async fn on_msg(&mut self, msg: GameMsg, game_handle: GameHandle) {
        let GameMsg { msg, id_user } = msg;
        let is_full = self.slots.iter().all(|slot| slot.is_some());

        match msg {
            ClientMsg::Chat(chat) => self.on_chat(id_user, chat).await,
            ClientMsg::Disconnect => self.on_disconnect(id_user).await,
            // Require the game to be full before a play can be made.
            ClientMsg::Play(play) if is_full => self.on_play(id_user, play, game_handle).await,
            _ => log::error!("unexpected message: {msg:?}"),
        }
    }
    /// Handles a chat message.
    async fn on_chat(&mut self, id_user: i32, chat: String) {
        if let Some(player) = self.player(id_user) {
            let msg = ServerMsg::Chat(player.clone(), chat.clone());

            // send the chat message to all other players.
            for slot in &self.slots {
                if let Some(Slot { game_player, .. }) = slot {
                    if let GamePlayer::User { id_user, .. } = game_player {
                        // Send the message to the player.
                        self.send_msg(*id_user, msg.clone());
                    }
                }
            }
        }
    }
    /// Handles a disconnect message.
    async fn on_disconnect(&mut self, id_user: i32) {
        let slot = self.slot_by_id_user_mut(id_user);

        if let Some(slot) = slot {
            if let GamePlayer::User { sender, .. } = &mut slot.game_player {
                // Set the sender half to `None`, which will disconnect the player.
                *sender = None;
            }
        }

        // send an update player message to the remaining users.
        self.send_all(ServerMsg::Players(
            self.slots
                .iter()
                .filter_map(|slot| slot.as_ref())
                .map(|slot| slot.player())
                .collect(),
        ));
    }
    /// Handles a play message.
    async fn on_play(&mut self, id_user: i32, play: Play, game_handle: GameHandle) {
        let slot = self
            .slot_by_id_user(id_user)
            .expect("player must be in the game");
        let player = slot.player();
        let player_num = slot.player_num;
        let to_play = match self.game.to_play() {
            Some(player_num) => player_num,
            // If `to_play` is None then the game is over.
            None => {
                self.send_msg(id_user, ServerMsg::Error(LiveError::Play(GameError::Over)));
                return;
            }
        };

        // Check whether it is the user's turn.
        if slot.player_num != to_play {
            self.send_msg(id_user, ServerMsg::Error(LiveError::NotYourTurn));
            return;
        }

        // Store the previous tiles so they can be sent to the users.
        let prev_tiles = Vec::from(self.game.board().grid_h().tiles());

        // Validate the play by testing it on the board.
        let fsm: &FastFsm = &self.fsm;
        match self.game.make_play(&play, fsm) {
            // Play is legal
            Ok(()) => {
                // increment the play count.
                self.plays += 1;

                // send a rack message to the player that made the play.
                let rack = self.game.player(player_num).rack();
                self.send_msg(id_user, ServerMsg::Rack(rack.iter().collect()));

                // send a play message to all players.
                self.send_all(ServerMsg::Play {
                    player,
                    prev_tiles,
                    play,
                    letter_bag_len: self.game.letter_bag_len(),
                    next: self.next_player(),
                });

                // make any neccesary ai plays.
                self.make_ai_plays();

                // start the move timer if the game is not over.
                self.start_move_timer(game_handle);
            }
            // Play is illegal
            Err(e) => {
                self.send_msg(id_user, ServerMsg::Error(LiveError::Play(e)));
            }
        }
    }
    /// Makes any AI plays as required.
    fn make_ai_plays(&mut self) {
        let fsm: &FastFsm = &self.fsm;

        // loop until the game is over.
        while let Some(to_play) = self.game.to_play() {
            if let Some(slot) = self.slot_by_player_num(to_play) {
                let difficulty = match slot.game_player {
                    // check whether the current player is an ai.
                    GamePlayer::Ai { difficulty, .. } => difficulty,
                    // check whether the current player has disconnected.
                    GamePlayer::User { sender: None, .. } => AiDifficulty::Easy,
                    // otherwise stop making moves.
                    _ => break,
                };
                let ai = match difficulty {
                    AiDifficulty::Easy => Ai::easy(),
                    AiDifficulty::Medium => Ai::medium(),
                    AiDifficulty::Hard => Ai::hard(),
                };
                let play = ai.next_play(fsm, &self.game);

                // store the previous tiles.
                let prev_tiles = Vec::from(self.game.board().grid_h().tiles());

                // increment the play count
                self.plays += 1;

                // make the play.
                self.game
                    .make_play(&play, fsm)
                    .expect("Ai play should never fail");

                // send a play message to all users.
                self.send_all(ServerMsg::Play {
                    player: slot.player(),
                    prev_tiles,
                    play,
                    letter_bag_len: self.game.letter_bag_len(),
                    next: self.next_player(),
                });
            }
        }
    }
    // If the game is not over, start a move timer.
    fn start_move_timer(&self, game_handle: GameHandle) {
        if let Some(to_play) = self.game.to_play() {
            let current_plays = self.plays;
            let id_user = self
                .slot_by_player_num(to_play)
                .and_then(|slot| slot.id_user())
                .expect("current player should be a user");

            tokio::spawn(async move {
                // wait `USER_TIMEOUT` seconds for the next player to make a play.
                tokio::time::sleep(*USER_TIMEOUT).await;

                // if the play count has not advanced, disconnect the user.
                let mut game = game_handle.lock().await;
                if game.plays == current_plays {
                    // send a timeout message to all users.
                    let player = game
                        .slot_by_id_user(id_user)
                        .map(|slot| slot.player())
                        .expect("slot to exist");
                    game.send_all(ServerMsg::Timeout(player));

                    // disconnect the user.
                    game.on_disconnect(id_user);
                }
            });
        }
    }
    /// Gets the next [`Player`].
    fn next_player(&self) -> Option<Player> {
        self.game
            .to_play()
            .and_then(|player_num| self.slot_by_player_num(player_num))
            .map(|slot| slot.player())
    }

    /// Gets the [`Player`] API type by id.
    fn player(&self, id_user: i32) -> Option<Player> {
        self.slot_by_id_user(id_user).map(|p| p.player())
    }
    /// Gets a reference to a slot by user id.
    fn slot_by_id_user(&self, id_user: i32) -> Option<&Slot> {
        self.slots
            .iter()
            .filter_map(|slot| slot.as_ref())
            .find(|slot| slot.has_id_user(id_user))
    }
    /// Gets a mutable reference to a slot by user id.
    fn slot_by_id_user_mut(&mut self, id_user: i32) -> Option<&mut Slot> {
        self.slots
            .iter_mut()
            .filter_map(|slot| slot.as_mut())
            .find(|slot| slot.has_id_user(id_user))
    }
    /// Gets a reference to a slot by [`PlayerNum`].
    fn slot_by_player_num(&self, player_num: PlayerNum) -> Option<&Slot> {
        self.slots
            .iter()
            .filter_map(|slot| slot.as_ref())
            .find(|slot| slot.player_num == player_num)
    }

    /// Attempts to send a message to the user.
    fn send_msg(&self, id_user: i32, msg: ServerMsg) {
        // Find the slot containing the player.
        let slot = self.slot_by_id_user(id_user);

        if let Some(slot) = slot {
            if let GamePlayer::User { sender, .. } = &slot.game_player {
                if let Some(sender) = sender {
                    if let Err(e) = sender.send(msg) {
                        log::error!("failed to send message: {e:?}");
                    }
                }
            }
        }
    }
    /// Attenpts to send a message to all players.
    fn send_all(&self, msg: ServerMsg) {
        self.slots
            .iter()
            .filter_map(|slot| slot.as_ref())
            .for_each(|slot| {
                if let GamePlayer::User { id_user, .. } = slot.game_player {
                    self.send_msg(id_user, msg.clone());
                }
            });
    }
}
