//! API types for the `api/users/*` routes.

use crate::auth::Auth;
use serde::{Deserialize, Serialize};

//--------------------------------------------
//               Utils
//--------------------------------------------
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

//--------------------------------------------
//               Login route
//--------------------------------------------
/// Request sent to the login endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    /// Username.
    pub username: String,
    /// Password (plaintext).
    pub password: String,
}

/// Response (200 OK) from the login endpoint.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginResponse {
    /// A token.
    pub auth: Auth,
    /// Information about the user account.
    pub user_details: UserDetails,
}

//--------------------------------------------
//               Create user route
//--------------------------------------------
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

/// Response (200 OK) sent from the signup endpoint.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignUpResponse {
    /// A token.
    pub auth: Auth,
    /// Information about the user account.
    pub user_details: UserDetails,
}

//--------------------------------------------
//               Get user details route
//--------------------------------------------
/// Response (200 OK) sent from the profile endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileResponse {
    /// A token (for the user that sent the request).
    pub auth: Auth,
    /// Information about the requested user account,
    pub user_details: UserDetails,
}

//--------------------------------------------
//               Update user info route
//--------------------------------------------
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

/// Response from updating a user account.
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAccountResponse {
    /// A token.
    pub auth: Auth,
}

//--------------------------------------------
//               Delete user route
//--------------------------------------------
/// Request to delete an account.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteAccount {
    /// The password of the account.
    pub password: String,
}
