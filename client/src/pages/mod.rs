//! Module containing implementations for each page.

mod friends;
mod games;
mod home;
mod invalid_state;
mod leaderboards;
mod live;
mod not_found;
mod users;

pub use friends::FriendsPage;
pub use games::{GameListPage, GameStatsPage};
pub use home::HomePage;
pub use invalid_state::InvalidStatePage;
pub use leaderboards::{FriendsLeaderboardPage, LeaderboardPage};
pub use live::LivePage;
pub use not_found::NotFoundPage;
pub use users::{AccountPage, LoginPage, ResetPasswordPage, ResetWithSecretPage, SignUpPage};
