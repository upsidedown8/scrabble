use crate::{
    db::Db,
    fsm::FsmHandle,
    handlers::live::player::{GamePlayer, Slot},
    models::{self, AiDifficulty},
};
use api::routes::live::{ClientMsg, LiveError, Player, ServerMsg};
use scrabble::{
    ai::Ai,
    error::GameError,
    game::{play::Play, tile::Tile, GameStatus, PlayerNum},
    util::fsm::FastFsm,
};
use std::{collections::HashMap, env, ops::Deref, sync::Arc, time::Duration};
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
    pub async fn create(
        db: Db,
        fsm: FsmHandle,
        ai_count: usize,
        player_count: usize,
        id_user: Option<i32>,
    ) -> Option<(i32, GameHandle)> {
        let total_count = ai_count + player_count;
        // create a queue that allows connected clients to send messages
        // to the game (multiple producers) and the game to receive the
        // message (single consumer).
        let (sender, mut receiver) = mpsc::unbounded_channel::<GameMsg>();
        // create a database record for the game.
        let id_game = models::Game::insert(&db).await.ok()?;
        // create database records for the ai players.
        let mut slots = vec![];
        for player_num in PlayerNum::iter(total_count).take(ai_count) {
            let difficulty = AiDifficulty::Medium;

            // insert a record for each ai player.
            let id_player = models::Player::insert_ai(&db, id_game, difficulty)
                .await
                .ok()?;
            slots.push(Some(Slot {
                id_player,
                player_num,
                game_player: GamePlayer::Ai { difficulty },
            }));
        }

        let game = Game {
            id_game,
            id_owner: id_user,
            plays: 0,
            game: scrabble::game::Game::with_players(total_count),
            slots,
            db,
            fsm,
            sender,
        };
        let game_handle = GameHandle(Arc::new(Mutex::new(game)));

        // Spawn a task to listen for messages and act on them.
        tokio::spawn({
            let game_handle = game_handle.clone();
            async move {
                while let Some(msg) = receiver.recv().await {
                    log::debug!("game message received: {msg:?}");
                    // lock the game and handle the message.
                    let mut game_lock = game_handle.lock().await;
                    game_lock.on_msg(msg, game_handle.clone()).await;
                }

                log::info!("closing game");
            }
        });

        Some((id_game, game_handle))
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

    /// Adds the player to the game, or returns false if adding the player failed.
    pub async fn add_player(&mut self, id_user: i32, tx: mpsc::UnboundedSender<ServerMsg>) -> bool {
        let player_count = self.slots.len();

        // loop until an empty slot is found
        for (player_num, slot) in PlayerNum::iter(player_count).zip(&mut self.slots) {
            // The player can join if the id matches or the slot is empty
            // and the player is a friend of the owner (if set).
            let (id_player, username) = match slot {
                Some(s) if s.id_user() == Some(id_user) => {
                    let row =
                        sqlx::query_file!("sql/live/find_player.sql", self.id_game(), id_user)
                            .execute(&self.db)
                            .await;
                    (row.id_player, row.username)
                }
                None => {
                    let row = sqlx::query_file!("sql/live/").execute(&self.db).await;
                    (row.id_player, row.username)
                }
                _ => continue,
            };

            // add the player to the game.
            *slot = Some(Slot {
                id_player,
                player_num,
                game_player: GamePlayer::User {
                    id_user,
                    username,
                    sender: Some(tx),
                },
            });

            // send a join game message.
            self.send_msg(
                id_user,
                ServerMsg::Joined {
                    id_game: self.id_game(),
                    id_player,
                    players: self.players(),
                    tiles: self.tiles(),
                    rack: self.rack(player_num),
                    scores: self.scores(),
                },
            );

            // send a message to update the players.
            self.send_all(ServerMsg::Players(self.players()));

            // Adding the user succeeded.
            return true;
        }

        // if no empty slot was found, return false.
        false
    }

    /// Handles a message from a client.
    async fn on_msg(&mut self, msg: GameMsg, game_handle: GameHandle) {
        let GameMsg { msg, id_user } = msg;
        let is_full = self.slots.iter().all(|slot| slot.is_some());

        match msg {
            ClientMsg::Chat(chat) => self.on_chat(id_user, chat),
            ClientMsg::Disconnect => self.on_disconnect(id_user),
            // Require the game to be full before a play can be made.
            ClientMsg::Play(play) if is_full => self.on_play(id_user, play, game_handle).await,
            _ => log::error!("unexpected message: {msg:?}"),
        }
    }
    /// Handles a chat message.
    fn on_chat(&mut self, id_user: i32, chat: String) {
        if let Some(player) = self.player(id_user) {
            self.send_all(ServerMsg::Chat(player, chat));
        }
    }
    /// Handles a disconnect message.
    fn on_disconnect(&mut self, id_user: i32) {
        let slot = self.slot_by_id_user_mut(id_user);

        if let Some(slot) = slot {
            if let GamePlayer::User { sender, .. } = &mut slot.game_player {
                // Set the sender half to `None`, which will disconnect the player.
                *sender = None;
            }
        }

        // send an update player message to the remaining users.
        self.send_all(ServerMsg::Players(self.players()));
    }
    /// Handles a play message.
    async fn on_play(&mut self, id_user: i32, play: Play, game_handle: GameHandle) {
        let slot = self
            .slot_by_id_user(id_user)
            .expect("player must be in the game");
        let id_player = slot.id_player;
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
        let prev_tiles = self.tiles();

        // Validate the play by testing it on the board.
        let fsm: &FastFsm = &self.fsm;
        match self.game.make_play(&play, fsm) {
            // Play is legal
            Ok(()) => {
                // increment the play count.
                self.plays += 1;

                // send a rack message to the player that made the play.
                self.send_msg(id_user, ServerMsg::Rack(self.rack(player_num)));

                // add the play to the database.
                self.insert_play(id_player, &play).await;

                // send a play message to all players.
                self.send_all(ServerMsg::Play {
                    player,
                    prev_tiles,
                    play,
                    letter_bag_len: self.game.letter_bag_len(),
                    next: self.next_player(),
                    scores: self.scores(),
                });

                // make any neccesary ai plays.
                self.make_ai_plays();

                match self.game.status() {
                    // start the move timer if the game is not over.
                    &GameStatus::ToPlay(to_play) => self.start_move_timer(to_play, game_handle),
                    // if the game is over, update the database records.
                    GameStatus::Over(game_over) => {
                        // set `is_over` to true.
                        sqlx::query_file!("sql/live/set_game_over.sql", self.id_game)
                            .execute(&self.db)
                            .await
                            .unwrap();

                        let winners: Vec<_> = game_over.winners().map(|(n, _)| n).collect();

                        // set the winners.
                        for player_num in PlayerNum::iter(self.slots.len()) {
                            if let Some(slot) = self.slot_by_player_num(player_num) {
                                let is_winner = winners.iter().any(|n| n == &player_num);

                                sqlx::query_file!(
                                    "sql/live/set_winner.sql",
                                    is_winner,
                                    slot.id_player,
                                )
                                .execute(&self.db)
                                .await
                                .unwrap();
                            }
                        }
                    }
                }
            }
            // Play is illegal
            Err(e) => {
                self.send_msg(id_user, ServerMsg::Error(LiveError::Play(e)));
            }
        }
    }
    /// Makes any AI plays as required.
    async fn make_ai_plays(&mut self) {
        let fsm: &FastFsm = &self.fsm;

        // loop until the game is over.
        while let Some(to_play) = self.game.to_play() {
            let slot = self.slot_by_player_num(to_play).expect("an occupied slot");
            let id_player = slot.id_player;
            let player = slot.player();
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
            let prev_tiles = self.tiles();

            // increment the play count
            self.plays += 1;

            // make the play.
            self.game
                .make_play(&play, fsm)
                .expect("Ai play should never fail");

            // add the play to the database.
            self.insert_play(id_player, &play).await;

            // send a play message to all users.
            self.send_all(ServerMsg::Play {
                player,
                prev_tiles,
                play,
                letter_bag_len: self.game.letter_bag_len(),
                next: self.next_player(),
                scores: self.scores(),
            });
        }
    }
    // If the game is not over, start a move timer.
    fn start_move_timer(&self, to_play: PlayerNum, game_handle: GameHandle) {
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
    /// Gets the next [`Player`].
    fn next_player(&self) -> Option<Player> {
        self.game
            .to_play()
            .and_then(|player_num| self.slot_by_player_num(player_num))
            .map(|slot| slot.player())
    }

    /// Gets the current scores.
    fn scores(&self) -> HashMap<Player, usize> {
        match self.game.status() {
            GameStatus::Over(game_over) => game_over
                .final_scores()
                .filter_map(|(player_num, score)| {
                    self.slot_by_player_num(player_num)
                        .map(|slot| slot.player())
                        .map(|player| (player, score))
                })
                .collect(),
            GameStatus::ToPlay(_) => PlayerNum::iter(self.slots.len())
                .map(|player_num| (player_num, self.game.player(player_num).score()))
                .filter_map(|(player_num, score)| {
                    self.slot_by_player_num(player_num)
                        .map(|slot| slot.player())
                        .map(|player| (player, score))
                })
                .collect(),
        }
    }
    /// Gets the rack tiles for a player.
    fn rack(&self, player_num: PlayerNum) -> Vec<Tile> {
        self.game.player(player_num).rack().iter().collect()
    }
    /// Gets the board tiles for the API.
    fn tiles(&self) -> Vec<Option<Tile>> {
        Vec::from(self.game.board().grid_h().tiles())
    }
    /// Gets the API data for all of the players.
    fn players(&self) -> Vec<Player> {
        self.slots
            .iter()
            .filter_map(|slot| slot.as_ref())
            .map(|slot| slot.player())
            .collect()
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
            if let GamePlayer::User {
                sender: Some(sender),
                ..
            } = &slot.game_player
            {
                if let Err(e) = sender.send(msg) {
                    log::error!("failed to send message: {e:?}");
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

    /// Inserts a play into the database.
    async fn insert_play(&self, id_player: i32, play: &Play) {
        match play {
            Play::Pass => todo!(),
            Play::Redraw(_) => todo!(),
            Play::Place(_) => todo!(),
        }

        todo!()
    }
    /// Inserts words for a play into the database.
    async fn insert_words(&self) {
        todo!()
    }
}
