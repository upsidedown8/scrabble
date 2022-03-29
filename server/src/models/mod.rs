//! Module that models database records.

mod friend_request;
mod game;
mod password_reset;
mod play;
mod player;
mod tile;
mod user;
mod word;

pub use friend_request::FriendRequest;
pub use game::Game;
pub use password_reset::PasswordReset;
pub use play::Play;
pub use player::Player;
pub use tile::Tile;
pub use user::User;
pub use word::Word;
