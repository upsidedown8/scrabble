//! Types relating to authentication.

use serde::{Deserialize, Serialize};

/// Auth token (JWT)
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Auth(pub String);
