//! Convenience methods for the friends api route.

use crate::{contexts::AuthSignal, error::Result, services::std_request_no_body};
use api::routes::friends::*;
use reqwasm::http::Method;

/// POST /api/friends [+Auth]
pub async fn add(auth_ctx: &AuthSignal, username: String) -> Result<()> {
    let url = format!("/friends/{username}");
    std_request_no_body(auth_ctx, &url, Method::POST).await
}

/// DELETE /api/friends/{username} [+Auth]
pub async fn remove(auth_ctx: &AuthSignal, username: String) -> Result<()> {
    let url = format!("/friends/{username}");
    std_request_no_body(auth_ctx, &url, Method::DELETE).await
}

/// GET /api/friends [+Auth]
pub async fn list(auth_ctx: &AuthSignal) -> Result<FriendsResponse> {
    std_request_no_body(auth_ctx, "/friends/requests", Method::GET).await
}

/// GET /api/friends/requests [+Auth]
pub async fn list_requests(auth_ctx: &AuthSignal) -> Result<FriendRequestsResponse> {
    std_request_no_body(auth_ctx, "/friends/requests", Method::GET).await
}
