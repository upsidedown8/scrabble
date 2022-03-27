//! Error types for the API.

use serde::{Deserialize, Serialize};

/// Response sent when a request fails.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorResponse {
    /// The http status.
    pub status: String,
    /// The reason for the error.
    pub msg: String,
}
