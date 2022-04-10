//! Types relating to authentication.

use serde::{Deserialize, Serialize};

/// Auth token (JWT)
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Token(pub String);

/// A wrapper for all response types that provides an optional
/// auth token.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct AuthWrapper<T> {
    /// The JWT.
    pub token: Option<Token>,
    /// The actual response data.
    pub response: T,
}
