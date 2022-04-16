use crate::{
    db::Db,
    fsm::FsmHandle,
    handlers::live::game::{GameHandle, GameMsg},
};
use std::{collections::HashMap, ops::Deref, sync::Arc, time::Duration};
use tokio::{sync::RwLock, time::interval};

/// The number of milliseconds waited between polls to close a game room.
const GAME_CLOSE_PERIOD: u64 = 10_000;

/// Type containing a thread-safe handle to all the games.
#[derive(Clone, Debug)]
pub struct GamesHandle(Arc<RwLock<Games>>);
impl GamesHandle {
    /// Creates a new `GamesHandle`.
    pub fn new(db: &Db, fsm: &FsmHandle) -> Self {
        let games_handle = GamesHandle(Arc::new(RwLock::new(Games {
            games: HashMap::default(),
            fsm: fsm.clone(),
            db: db.clone(),
        })));

        // Spawn a task that closes Games which have no remaining players
        tokio::spawn({
            let games_handle = games_handle.clone();
            async move {
                let mut interval = interval(Duration::from_millis(GAME_CLOSE_PERIOD));

                // repeat while the application is running, closing empty
                // games at a fixed interval.
                loop {
                    interval.tick().await;

                    let mut games = games_handle.write().await;
                    let mut to_remove = vec![];

                    // iterate over the games and find any that are empty.
                    for (&id_game, game_handle) in games.games.iter() {
                        let game = game_handle.lock().await;
                        if game.is_empty() {
                            // send a close message to the game, which stops the async task
                            // that is listening for messages.
                            game.sender().send(GameMsg::Close).unwrap();
                            to_remove.push(id_game);
                        }
                    }

                    // remove the games from the hashmap.
                    for id_game in to_remove {
                        log::info!("removing empty game: {id_game}");
                        games.games.remove(&id_game);
                    }
                }
            }
        });

        games_handle
    }
}
impl Deref for GamesHandle {
    type Target = RwLock<Games>;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug)]
pub struct Games {
    games: HashMap<i32, GameHandle>,
    fsm: FsmHandle,
    db: Db,
}
impl Games {
    /// Gets a reference to the Fsm.
    pub fn fsm(&self) -> FsmHandle {
        self.fsm.clone()
    }
    /// Gets a database connection handle.
    pub fn db(&self) -> Db {
        self.db.clone()
    }
    /// Gets a reference to a game.
    pub fn get(&self, id_game: i32) -> Option<GameHandle> {
        self.games.get(&id_game).cloned()
    }
    /// Inserts a game into the list of games.
    pub async fn insert(
        &mut self,
        ai_count: usize,
        player_count: usize,
        id_owner: Option<i32>,
    ) -> Option<GameHandle> {
        log::info!("inserting game");

        let db = self.db();
        let fsm = self.fsm();
        let created = GameHandle::create(db, fsm, ai_count, player_count, id_owner).await;

        if let Some((id_game, game_handle)) = created {
            self.games.insert(id_game, game_handle.clone());
            Some(game_handle)
        } else {
            None
        }
    }
}
