use crate::{
    db::Db,
    fsm::FsmHandle,
    models::{self, AiDifficulty},
};
use api::routes::live::{ClientMsg, LiveError, Player, ServerMsg};
use chrono::Utc;
use scrabble::{
    ai::Ai,
    error::GameError,
    game::{play::Play, tile::Tile, GameOver, GameStatus, PlayerNum},
    util::{bitboard::BitBoard, fsm::FastFsm, scoring, words::WordsExt},
};
use std::{
    collections::{hash_map::Entry, HashMap},
    env,
    ops::Deref,
    sync::Arc,
    time::Duration,
};
use tokio::sync::{mpsc, Mutex};

lazy_static::lazy_static! {
    static ref USER_TIMEOUT: Duration = {
        let seconds = env::var("USER_TIMEOUT").expect("`USER_TIMEOUT` env var");
        let seconds = seconds.parse().expect("`USER_TIMEOUT` should be an integer");

        Duration::from_secs(seconds)
    };
}

/// A thread-safe handle to a game.
#[derive(Clone, Debug)]
pub struct GameHandle(Arc<Mutex<Game>>);
impl Deref for GameHandle {
    type Target = Mutex<Game>;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
impl GameHandle {
    /// Creates a new [`GameHandle`] from the player count, returning
    /// the id of the game.
    pub async fn create(
        db: Db,
        fsm: FsmHandle,
        ai_count: usize,
        player_count: usize,
        id_owner: Option<i32>,
    ) -> Option<(i32, GameHandle)> {
        let total_count = ai_count + player_count;
        // create a queue that allows connected clients to send messages
        // to the game (multiple producers) and the game to receive the
        // message (single consumer).
        let (sender, mut receiver) = mpsc::unbounded_channel::<GameMsg>();
        // create a database record for the game.
        let id_game = models::Game::insert(&db).await.ok()?;
        // create database records for the ai players.
        let mut slots = HashMap::default();
        // add the ai players at the end of the slots, to allow the human players to go first.
        for player_num in PlayerNum::iter(total_count)
            .skip(player_count)
            .take(ai_count)
        {
            let difficulty = AiDifficulty::Medium;

            log::trace!("inserting ai player");

            // insert a record for each ai player.
            let id_player = models::Player::insert_ai(&db, id_game, difficulty)
                .await
                .ok()?;

            // insert into slots hashmap.
            slots.insert(
                player_num,
                Slot {
                    id_player,
                    game_player: GamePlayer::Ai { difficulty },
                },
            );
        }

        // create the game.
        let game = Game {
            game: scrabble::game::Game::new(total_count),
            play_count: 0,
            slots,
            db,
            fsm,
            id_game,
            id_owner,
            sender,
        };
        let game_handle = GameHandle(Arc::new(Mutex::new(game)));

        // spawn a task to listen for messages and act on them.
        tokio::spawn({
            let game_handle = game_handle.clone();
            async move {
                while let Some(msg) = receiver.recv().await {
                    log::debug!("game message received: {msg:?}");
                    // lock the game and handle the message.
                    let mut game_lock = game_handle.lock().await;
                    game_lock.on_msg(msg, game_handle.clone()).await;
                }

                log::info!("closing game: {id_game}");
            }
        });

        Some((id_game, game_handle))
    }
}

/// Handles live games.
#[derive(Debug)]
pub struct Game {
    game: scrabble::game::Game,
    play_count: usize,
    slots: HashMap<PlayerNum, Slot>,

    db: Db,
    fsm: FsmHandle,

    id_game: i32,
    id_owner: Option<i32>,

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

    /// Attempts to add a player to the game. Return value indicates
    /// success.
    pub async fn add_player(&mut self, id_user: i32, tx: mpsc::UnboundedSender<ServerMsg>) -> bool {
        let id_game = self.id_game();

        // first check whether the user is in the game but
        // has disconnected.
        for player_num in self.game.player_nums() {
            if let Entry::Occupied(mut e) = self.slots.entry(player_num) {
                let slot = e.get_mut();

                // check whether the slot contains a player with the same
                // user id.
                if Some(id_user) == slot.id_user() {
                    // update the `tx` field of the user.
                    slot.set_sender(tx);

                    // since the user was previously added, no database operation
                    // is required.

                    // Notify the players.
                    self.send_join_msg(player_num);
                    return true;
                }
            }
        }

        // then try adding the user to a vacant position.
        for player_num in self.game.player_nums() {
            // If the slot is vacant, add the player.
            if let Entry::Vacant(e) = self.slots.entry(player_num) {
                let id_owner = self.id_owner;
                // add a player record in the database if the user is
                // a friend of `self.id_owner` (or `self.id_owner` is
                // None).
                match models::Player::insert_user(&self.db, id_game, id_user, id_owner).await {
                    Ok((id_player, username)) => {
                        // Insert the player.
                        e.insert(Slot {
                            id_player,
                            game_player: GamePlayer::User {
                                id_user,
                                username,
                                sender: Some(tx),
                            },
                        });

                        // Notify the players.
                        self.send_join_msg(player_num);
                        return true;
                    }
                    Err(e) => {
                        log::error!("failed to insert user: {e:?}");

                        // exit the loop.
                        break;
                    }
                }
            }
        }

        // send a message saying that joining failed.
        if let Err(e) = tx.send(ServerMsg::Error(LiveError::FailedToJoin)) {
            log::error!("failed to send message to user: {e:?}");
        }

        false
    }
    /// Sends a join message to the player that joined and notifies
    /// all other players in the game.
    fn send_join_msg(&self, player_num: PlayerNum) {
        // send a join game message.
        let slot = &self.slots[&player_num];
        slot.send_msg(ServerMsg::Joined {
            id_game: self.id_game(),
            id_player: slot.id_player(),
            capacity: self.game.player_count(),
            players: self.api_players(),
            tiles: self.api_tiles(),
            rack: self.api_rack(player_num),
            scores: self.api_scores(),
            next: self.api_next(),
        });

        // send a message to update the players.
        self.send_all(ServerMsg::Players(self.api_players()));
        self.send_all(ServerMsg::UserConnected(slot.player()));

        // check whether there are enough players to start and
        // no plays have been made yet.
        if self.play_count == 0 && self.is_full() {
            self.send_all(ServerMsg::Starting);
        }
    }

    /// Called when a message is received from a user.
    async fn on_msg(&mut self, msg: GameMsg, game_handle: GameHandle) {
        let GameMsg { msg, id_user } = msg;

        match msg {
            ClientMsg::Disconnect => self.on_disconnect(id_user),
            ClientMsg::Chat(chat) => self.on_chat(id_user, chat),
            // Require the game to be full before a play can be made.
            ClientMsg::Play(play) if self.is_full() => {
                self.on_play(id_user, play, game_handle).await
            }
            _ => log::error!("unexpected message: {msg:?}"),
        }
    }
    /// Called when a chat message is received.
    fn on_chat(&self, id_user: i32, chat: String) {
        if let Some(player_num) = self.id_user_to_player_num(id_user) {
            let slot = &self.slots[&player_num];
            let player = slot.player();

            self.send_all(ServerMsg::Chat(player, chat));
        }
    }
    /// Called when a disconnect message is received.
    fn on_disconnect(&mut self, id_user: i32) {
        if let Some(player_num) = self.id_user_to_player_num(id_user) {
            // send a disconnect message.
            let slot = self.slots.get_mut(&player_num).unwrap();
            let player = slot.player();

            // disconnect the user.
            slot.disconnect();

            // send a message containing the new players.
            self.send_all(ServerMsg::Players(self.api_players()));
            self.send_all(ServerMsg::UserDisconnected(player));
        }
    }
    /// Called when a play message is received.
    async fn on_play(&mut self, id_user: i32, play: Play, game_handle: GameHandle) {
        let to_play = self.game.to_play();
        let player_num = self.id_user_to_player_num(id_user).unwrap();
        let slot = &self.slots[&player_num];

        // check whether the game is over.
        if to_play.is_none() {
            slot.send_msg(ServerMsg::Error(LiveError::Play(GameError::Over)));
            return;
        }

        // check whether it is the player's turn.
        if to_play != Some(player_num) {
            slot.send_msg(ServerMsg::Error(LiveError::NotYourTurn));
            return;
        }

        // attempt to make the play.
        if self.try_play(play, player_num).await {
            // make plays for any ai players.
            self.make_ai_plays().await;

            log::trace!("finished AI plays");

            match self.game.status() {
                // start a move timer if the game is ongoing.
                &GameStatus::ToPlay(to_play) => self.start_timer(to_play, game_handle),
                // when the game ends, update the database records.
                GameStatus::Over(game_over) => self.on_game_over(game_over).await,
            }
        }
    }
    /// Handles database updates for the end of the game.
    async fn on_game_over(&self, game_over: &GameOver) {
        // set the `is_over` column to true and the `end_time` column
        // to the current time on `tbl_game`.
        let end_time = Utc::now().naive_utc();
        sqlx::query_file!("sql/live/set_game_over.sql", self.id_game(), end_time)
            .execute(&self.db)
            .await
            .unwrap();

        // set `is_winner` to true for all winners.
        for (player_num, _) in game_over.winners() {
            let id_player = self.slots[&player_num].id_player();
            sqlx::query_file!("sql/live/set_winner.sql", id_player)
                .execute(&self.db)
                .await
                .unwrap();
        }

        // set `is_winner` to false for all losers.
        for (player_num, _) in game_over.losers() {
            let id_player = self.slots[&player_num].id_player();
            sqlx::query_file!("sql/live/set_loser.sql", id_player)
                .execute(&self.db)
                .await
                .unwrap();
        }

        self.send_all(ServerMsg::Over(game_over.reason()));
    }
    /// Continues to make plays for Ai players until a connected
    /// player is encountered or the game ends.
    async fn make_ai_plays(&mut self) {
        // loop until game is over.
        while let Some(to_play) = self.game.to_play() {
            if let Some(ai) = &self.slots[&to_play].ai() {
                let fsm: &FastFsm = &self.fsm;
                log::trace!("finding next play");
                let play = ai.next_play(fsm, &self.game);
                let is_success = self.try_play(play, to_play).await;

                log::trace!("made ai play");

                assert!(is_success, "Ai move should always be valid");
            } else {
                // No more ai plays need to be made.
                break;
            }
        }
    }
    /// Attempts to make a play. Return value indicates success.
    async fn try_play(&mut self, play: Play, player_num: PlayerNum) -> bool {
        log::info!("attempting to make play: {play:?}");

        // store the current tile positions.
        let prev_tiles = self.api_tiles();
        let fsm: &FastFsm = &self.fsm;
        match self.game.make_play(&play, fsm) {
            Ok(()) => {
                log::trace!("legal play");

                self.play_count += 1;

                // add the play to the database.
                self.insert_play(&play, player_num).await;

                // send a rack message.
                self.slots[&player_num].send_msg(ServerMsg::Rack(self.api_rack(player_num)));
                // send a play message to all players.
                self.send_all(ServerMsg::Play {
                    player: self.api_player(player_num).unwrap(),
                    prev_tiles,
                    play,
                    letter_bag_len: self.game.letter_bag_len(),
                    next: self.api_next(),
                    scores: self.api_scores(),
                });

                true
            }
            Err(e) => {
                log::debug!("illegal play");

                // send a message for an illegal play.
                self.slots[&player_num].send_msg(ServerMsg::Error(LiveError::Play(e)));
                false
            }
        }
    }
    /// Adds a play to the database.
    async fn insert_play(&mut self, play: &Play, player_num: PlayerNum) {
        let slot = &self.slots[&player_num];
        let id_player = slot.id_player;

        // insert a play record.
        log::trace!("inserting play");
        let id_play = models::Play::insert(&self.db, id_player).await.unwrap();

        // insert records for each of the placed tiles.
        let mut new_h = BitBoard::default();
        let mut new_v = BitBoard::default();
        if let Play::Place(tile_positions) = play {
            for (pos, tile) in tile_positions {
                models::Tile::insert(&self.db, id_play, pos, tile)
                    .await
                    .unwrap();

                new_h.set(*pos);
                new_v.set(pos.swap_rc());
            }
        }

        // insert records for each of the vertical and horizontal words.
        for (letters, score) in self.words(new_h, new_v) {
            models::Word::insert(&self.db, id_play, letters, score)
                .await
                .unwrap();
        }
    }

    /// Iterates over the new (word, score) tuples on the board.
    fn words(
        &self,
        new_h: BitBoard,
        new_v: BitBoard,
    ) -> impl Iterator<Item = (String, usize)> + '_ {
        let board = self.game.board();
        let horizontal = board
            .grid_h()
            .word_boundaries()
            .intersecting(new_h)
            .words(board.grid_h())
            .map(move |word| {
                let word_str = word.to_string();
                let score = scoring::score_unchecked(word, &new_h);
                (word_str, score)
            });
        let vertical = board
            .grid_v()
            .word_boundaries()
            .intersecting(new_v)
            .words(board.grid_v())
            .map(move |word| {
                let word_str = word.to_string();
                let score = scoring::score_unchecked(word, &new_v);
                (word_str, score)
            });

        horizontal.chain(vertical)
    }
    /// Starts a move timer for the specified player.
    fn start_timer(&self, player_num: PlayerNum, game_handle: GameHandle) {
        let curr_count = self.play_count;
        let id_user = self.slots[&player_num].id_user().unwrap();

        tokio::spawn(async move {
            // wait `USER_TIMEOUT` seconds for the next player to make a play.
            tokio::time::sleep(*USER_TIMEOUT).await;

            let mut game = game_handle.lock().await;
            // if the play count has not advanced, disconnect the user.
            if game.play_count == curr_count {
                // send a timeout message to all users.
                let player = game
                    .id_user_to_player_num(id_user)
                    .and_then(|player_num| game.api_player(player_num))
                    .expect("player to exist");
                game.send_all(ServerMsg::Timeout(player));

                // disconnect the user.
                game.on_disconnect(id_user);
            }
        });
    }

    /// Sends a message to all users.
    fn send_all(&self, msg: ServerMsg) {
        log::trace!("sending message to all: {msg:?}");
        for slot in self.slots.values() {
            slot.send_msg(msg.clone());
        }
    }

    /// Gets the number of slots.
    fn slot_count(&self) -> usize {
        self.game.player_count()
    }
    /// Gets the number of occupied slots.
    fn occupied_count(&self) -> usize {
        self.slots.len()
    }
    /// Checks whether the game is full.
    fn is_full(&self) -> bool {
        self.slot_count() == self.occupied_count()
    }
    /// Finds `PlayerNum` by user id.
    fn id_user_to_player_num(&self, id_user: i32) -> Option<PlayerNum> {
        self.slots
            .iter()
            .find(|(_, slot)| slot.id_user() == Some(id_user))
            .map(|(player_num, _)| player_num)
            .copied()
    }
    /// Gets the score for a player.
    fn score(&self, player_num: PlayerNum) -> usize {
        // If the game is over a bonus may be applied, so check
        // whether this is the case to find the score.
        match self.game.status() {
            GameStatus::ToPlay(_) => self.game.player(player_num).score(),
            GameStatus::Over(game_over) => game_over.score(player_num),
        }
    }

    /// Gets the rack tiles API type for a player.
    fn api_rack(&self, player_num: PlayerNum) -> Vec<Tile> {
        self.game.player(player_num).rack().tiles().collect()
    }
    /// Gets the API type for the next player.
    fn api_next(&self) -> Option<Player> {
        self.game
            .to_play()
            .and_then(|player_num| self.api_player(player_num))
    }
    /// Gets a struct representing a player for the API.
    fn api_player(&self, player_num: PlayerNum) -> Option<Player> {
        self.slots.get(&player_num).map(|slot| slot.player())
    }
    /// Gets a list of players for the API.
    fn api_players(&self) -> Vec<Player> {
        self.game
            .player_nums()
            .flat_map(|player_num| self.api_player(player_num))
            .collect()
    }
    /// Gets a HashMap storing scores for the API.
    fn api_scores(&self) -> HashMap<Player, usize> {
        self.game
            .player_nums()
            .flat_map(|player_num| {
                self.api_player(player_num)
                    .map(|api_player| (api_player, self.score(player_num)))
            })
            .collect()
    }
    /// Gets the board tiles for the API.
    fn api_tiles(&self) -> Vec<Option<Tile>> {
        Vec::from(self.game.board().grid_h().tiles())
    }
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

/// One of a fixed number of slots in a game.
#[derive(Debug)]
pub struct Slot {
    /// Database id of the player in the slot.
    id_player: i32,
    /// The actual player, either Ai or a connected user.
    game_player: GamePlayer,
}
impl Slot {
    /// Gets the player id.
    pub fn id_player(&self) -> i32 {
        self.id_player
    }
    /// Gets the optional user id.
    pub fn id_user(&self) -> Option<i32> {
        match self.game_player {
            GamePlayer::User { id_user, .. } => Some(id_user),
            _ => None,
        }
    }

    /// Gets the the Ai for the player if the player is an Ai or
    /// a disconnected user.
    pub fn ai(&self) -> Option<Ai> {
        match self.game_player {
            GamePlayer::Ai { difficulty } => Some(match difficulty {
                AiDifficulty::Easy => Ai::easy(),
                AiDifficulty::Medium => Ai::medium(),
                AiDifficulty::Hard => Ai::hard(),
            }),
            GamePlayer::User { sender: None, .. } => Some(Ai::easy()),
            _ => None,
        }
    }

    /// Updates the `sender` of the contained user.
    pub fn set_sender(&mut self, tx: mpsc::UnboundedSender<ServerMsg>) {
        if let GamePlayer::User { sender, .. } = &mut self.game_player {
            *sender = Some(tx);
        }
    }

    /// Sends a message to the user if they are connected.
    pub fn send_msg(&self, msg: ServerMsg) {
        if let GamePlayer::User {
            sender: Some(sender),
            ..
        } = &self.game_player
        {
            log::trace!("send message: {msg:?}");
            if let Err(e) = sender.send(msg) {
                log::error!("failed to send message: {e:?}");
            }
        }
    }
    /// Disconnects the user.
    pub fn disconnect(&mut self) {
        if let GamePlayer::User { sender, .. } = &mut self.game_player {
            // Set the sender half to `None`, which will disconnect the player.
            *sender = None;
        }
    }

    /// Gets the API Player type for an occupied slot.
    pub fn player(&self) -> Player {
        Player {
            id_player: self.id_player,
            username: match &self.game_player {
                GamePlayer::Ai { difficulty, .. } => String::from(match difficulty {
                    AiDifficulty::Easy => "AI (easy)",
                    AiDifficulty::Medium => "AI (medium)",
                    AiDifficulty::Hard => "AI (hard)",
                }),
                GamePlayer::User {
                    username, sender, ..
                } => match sender.is_some() {
                    true => username.clone(),
                    false => format!("Disconnected ({username})"),
                },
            },
        }
    }
}

/// Either an AI player or a connected user.
#[derive(Debug)]
pub enum GamePlayer {
    /// The player is an AI.
    Ai {
        /// The Ai difficulty.
        difficulty: AiDifficulty,
    },
    /// The player is a connected user.
    User {
        /// Id of the connected user.
        id_user: i32,
        /// Username of the player.
        username: String,
        /// Sender half of an mpsc queue that sends `ServerMsg`s to
        /// a connected user. If `None` the player has disconnected,
        /// and an easy AI will make their moves until they reconnect.
        sender: Option<mpsc::UnboundedSender<ServerMsg>>,
    },
}
