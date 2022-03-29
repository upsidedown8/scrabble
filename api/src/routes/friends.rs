//! API types for /friends.

use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

/// A list of friends for a user.
#[derive(Debug, Serialize, Deserialize)]
pub struct FriendsResponse {
    /// The friends of a user.
    pub friends: Vec<Friend>,
}

/// A list of potential friends for a user.
#[derive(Debug, Serialize, Deserialize)]
pub struct FriendRequestsResponse {
    /// The people that have sent friend requests to a user.
    pub requests: Vec<Friend>,
}

/// A friend that has been added, or someone who has sent
/// the user a friend request.
#[derive(Debug, Serialize, Deserialize)]
pub struct Friend {
    /// Username of the friend.
    pub username: String,
    /// Date that the request was sent / accepted.
    pub since: NaiveDateTime,
}
