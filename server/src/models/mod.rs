use std::convert::Infallible;
use warp::Filter;

pub mod game;
pub mod play;
pub mod player;
pub mod tile;
pub mod user;
pub mod word;

pub type Db = sqlx::SqlitePool;

/// Provides each request handler with access to the database connection pool.
pub fn with_db(db: &Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    let db = db.clone();
    warp::any().map(move || db.clone())
}
