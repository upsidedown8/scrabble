use crate::{db::Db, fsm::FsmHandle, handlers::live::game::GameHandle};
use std::{collections::HashMap, ops::Deref, sync::Arc};
use tokio::sync::RwLock;

/// Type containing a thread-safe handle to all the games.
#[derive(Clone, Debug)]
pub struct GamesHandle(Arc<RwLock<Games>>);
impl GamesHandle {
    /// Creates a new `GamesHandle`.
    pub fn new(db: &Db, fsm: &FsmHandle) -> Self {
        let games = Games::new(db, fsm);

        GamesHandle(Arc::new(RwLock::new(games)))
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
    /// Creates a new list of games.
    pub fn new(db: &Db, fsm: &FsmHandle) -> Self {
        Self {
            games: HashMap::new(),
            fsm: fsm.clone(),
            db: db.clone(),
        }
    }
    /// Gets a reference to the Fsm.
    pub fn fsm(&self) -> FsmHandle {
        self.fsm.clone()
    }
    /// Gets a database connection handle.
    pub fn db(&self) -> Db {
        self.db.clone()
    }
    /// Inserts a game into the list of games.
    pub async fn create_game(
        &mut self,
        ai_count: usize,
        player_count: usize,
        id_user: Option<i32>,
    ) -> Option<GameHandle> {
        if let Some((id_game, game_handle)) =
            GameHandle::create(self.db(), self.fsm(), ai_count, player_count, id_user).await
        {
            self.games.insert(id_game, game_handle.clone());
            Some(game_handle)
        } else {
            None
        }
    }
    /// Gets a reference to a game.
    pub fn get_game(&self, id_game: i32) -> Option<GameHandle> {
        self.games.get(&id_game).cloned()
    }
}
