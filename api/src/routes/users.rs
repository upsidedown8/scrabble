//! API types for /users.

use serde::{Deserialize, Serialize};

/// Struct storing common user information.
#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserDetails {
    /// Username.
    pub username: String,
    /// Email address.
    pub email: String,
    /// Whether the account is private.
    pub is_private: bool,
}

/// Data about a user that can be publically accessed.
#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    /// Username.
    pub username: String,
}

/// Request sent to reset a password. If it succeeds, a message
/// is sent to the user's email address.
#[derive(Debug, Serialize, Deserialize)]
pub struct ResetPassword {
    /// The account username.
    pub username: String,
}

/// Request sent to update a password after the secret
/// has been recieved by email.
#[derive(Debug, Serialize, Deserialize)]
pub struct ResetWithSecret {
    /// Random hexadecimal secret.
    pub secret_hex: String,
    /// The new password for the account.
    pub new_password: String,
    /// The username for the account.
    pub username: String,
}

/// Request sent to the login endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    /// Username.
    pub username: String,
    /// Password (plaintext).
    pub password: String,
}

/// Request sent to the signup endpoint. All fields are validated
/// on the server side.
#[derive(Debug, Serialize, Deserialize)]
pub struct SignUp {
    /// Username.
    pub username: String,
    /// Email.
    pub email: String,
    /// Password.
    pub password: String,
    /// Whether to make the account private.
    pub is_private: bool,
}

/// A request to update a user account.
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAccount {
    /// The previous password.
    pub old_password: String,
    /// Optional new email address.
    pub email: Option<String>,
    /// Optional new username.
    pub username: Option<String>,
    /// Optional new password.
    pub password: Option<String>,
    /// Optionally specify Whether to make the account private.
    pub is_private: Option<bool>,
}

/// Request to delete an account.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteAccount {
    /// The password of the account.
    pub password: String,
}
