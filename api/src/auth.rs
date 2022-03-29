//! Types relating to authentication.

use serde::{Deserialize, Serialize};

/// Auth token (JWT)
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Auth(pub String);

/// A wrapper for all response types that provides an optional
/// auth token.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct AuthWrapper<T> {
    /// The JWT.
    pub auth: Option<Auth>,
    /// The actual response data.
    pub response: T,
}
