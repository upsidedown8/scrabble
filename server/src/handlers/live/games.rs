use crate::{db::Db, fsm::FsmHandle, handlers::live::game::GameHandle};
use std::{collections::HashMap, ops::Deref, sync::Arc};
use tokio::sync::RwLock;

/// Type containing a thread-safe handle to all the games.
#[derive(Clone, Debug)]
pub struct GamesHandle(Arc<RwLock<Games>>);
impl GamesHandle {
    /// Creates a new `GamesHandle`.
    pub fn new(db: &Db, fsm: &FsmHandle) -> Self {
        GamesHandle(Arc::new(RwLock::new(Games {
            games: HashMap::default(),
            fsm: fsm.clone(),
            db: db.clone(),
        })))
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
