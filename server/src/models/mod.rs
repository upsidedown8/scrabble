use std::convert::Infallible;
use warp::Filter;

mod friend;
mod friend_request;
mod game;
mod outcome;
mod password_reset;
mod play;
mod player;
mod tile;
mod user;
mod word;

pub use friend::Friend;
pub use friend_request::FriendRequest;
pub use game::Game;
pub use outcome::Outcome;
pub use password_reset::PasswordReset;
pub use play::Play;
pub use player::Player;
pub use tile::Tile;
pub use user::User;
pub use word::Word;

pub type Db = sqlx::SqlitePool;

/// Provides each request handler with access to the database connection pool.
pub fn with_db(db: &Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    let db = db.clone();
    warp::any().map(move || db.clone())
}
