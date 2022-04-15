//! Reuseable components used across pages.

mod board;
mod chat;
mod counter;
mod error_msg;
mod fa_icon;
mod footer;
mod friends_table;
mod leaderboard;
mod navbar;
mod progress;
mod rack;
mod separator;
mod tile;

pub use board::Board;
pub use chat::{Chat, Msg};
pub use counter::{Counter, FixedCounter};
pub use error_msg::{ErrorMsg, StaticErrorMsg};
pub use fa_icon::FaIcon;
pub use footer::Footer;
pub use friends_table::FriendsTable;
pub use leaderboard::Leaderboard;
pub use navbar::Navbar;
pub use progress::Progress;
pub use rack::Rack;
pub use separator::Separator;
pub use tile::Tile;
