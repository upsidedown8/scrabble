//! Reuseable components used across pages.

mod error_msg;
mod fa_icon;
mod footer;
mod leaderboard;
mod navbar;
mod progress;
mod separator;

pub use error_msg::{ErrorMsg, StaticErrorMsg};
pub use fa_icon::FaIcon;
pub use footer::Footer;
pub use leaderboard::Leaderboard;
pub use navbar::Navbar;
pub use progress::Progress;
pub use separator::Separator;
