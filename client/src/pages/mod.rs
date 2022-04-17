//! Module containing implementations for each page.

mod friends;
mod games;
mod invalid_state;
mod leaderboards;
mod live;
mod not_found;
mod users;

use chrono::NaiveDateTime;
pub use friends::FriendsPage;
pub use games::{GameListPage, GameStatsPage};
pub use invalid_state::InvalidStatePage;
pub use leaderboards::{FriendsLeaderboardPage, LeaderboardPage};
pub use live::LivePage;
pub use not_found::NotFoundPage;
pub use users::{AccountPage, LoginPage, ResetPasswordPage, ResetWithSecretPage, SignUpPage};

/// Formats a NaiveDateTime.
pub fn format_datetime(dt: NaiveDateTime) -> String {
    dt.format("%a %d %b %Y (%H:%M:%S)").to_string()
}

/// Rounds a number to 2dp.
pub fn format_f32(f: f32) -> String {
    format!("{f:.2}")
}

/// Formats a boolean as a user-readable string.
pub fn format_bool(b: bool) -> &'static str {
    match b {
        true => "Yes",
        false => "No",
    }
}
