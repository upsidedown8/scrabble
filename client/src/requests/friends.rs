//! Convenience methods for the friends api route.

use crate::{context::AuthSignal, error::Result, requests::req_no_body};
use api::routes::friends::*;
use reqwasm::http::Method;

/// POST /api/friends [+Auth]
pub async fn add(auth_signal: &AuthSignal, username: String) -> Result<()> {
    req_no_body(
        &format!("/friends/{username}"),
        Method::POST,
        Some(auth_signal),
    )
    .await
}

/// DELETE /api/friends/{username} [+Auth]
pub async fn remove(auth_signal: &AuthSignal, username: String) -> Result<()> {
    req_no_body(
        &format!("/friends/{username}"),
        Method::DELETE,
        Some(auth_signal),
    )
    .await
}

/// GET /api/friends [+Auth]
pub async fn list(auth_signal: &AuthSignal) -> Result<FriendsResponse> {
    req_no_body(&format!("/friends"), Method::GET, Some(auth_signal)).await
}

/// GET /api/friends/requests [+Auth]
pub async fn list_requests(auth_signal: &AuthSignal) -> Result<FriendRequestsResponse> {
    req_no_body(
        &format!("/friends/requests"),
        Method::GET,
        Some(auth_signal),
    )
    .await
}
