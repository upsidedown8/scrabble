use crate::{db::Db, fsm::FsmHandle, handlers::live::player::Player};
use api::routes::live::{ClientMsg, ServerMsg};
use std::{
    ops::Deref,
    sync::{
        atomic::{AtomicI32, Ordering},
        Arc,
    },
};
use tokio::sync::{mpsc, Mutex};

static GAME_ID: AtomicI32 = AtomicI32::new(0);

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
        // create a queue that allows connected clients to send messages
        // to the game (multiple producers) and the game to receive the
        // message (single consumer).
        let (sender, mut receiver) = mpsc::unbounded_channel::<GameMsg>();

        let id_game = GAME_ID.fetch_add(1, Ordering::Relaxed);
        let game = Game {
            id_game,
            sender,
            players: vec![],
            db,
            fsm,
            game: scrabble::game::Game::with_players(4),
        };
        let game_handle = GameHandle(Arc::new(Mutex::new(game)));

        // Spawn a task to listen for messages and act on them.
        tokio::spawn({
            let game_handle = game_handle.clone();
            async move {
                while let Some(msg) = receiver.recv().await {
                    log::debug!("game message received: {msg:?}");
                    // lock the game and handle the message.
                    // let mut game_lock = game_handle.lock().await;
                    // game_lock.on_msg(msg).await;
                }

                log::info!("closing game");
            }
        });

        (id_game, game_handle)
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
    /// Clients can use this sender to send
    /// messages to the game.
    sender: mpsc::UnboundedSender<GameMsg>,
    /// The players in the game.
    players: Vec<Player>,
    /// Database pool.
    db: Db,
    /// Handle to Fsm.
    fsm: FsmHandle,
    /// Game state.
    game: scrabble::game::Game,
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
    /// Adds a player to the game.
    pub fn add_player(&mut self, id_user: i32, tx: mpsc::UnboundedSender<ServerMsg>) {
        self.players.push(Player::Human(Some(tx)));
    }
}
