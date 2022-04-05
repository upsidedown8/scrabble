//! Module containing code for displaying pages.

mod account;
mod home;
mod login;
mod not_found;
mod play;
mod reset_password;
mod reset_with_secret;
mod sign_up;

pub use account::AccountPage;
pub use home::HomePage;
pub use login::LoginPage;
pub use not_found::NotFoundPage;
pub use play::PlayPage;
pub use reset_password::ResetPasswordPage;
pub use reset_with_secret::ResetWithSecretPage;
pub use sign_up::SignUpPage;
