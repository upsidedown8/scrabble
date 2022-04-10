//! Convenience methods for the leaderboard api route.

use crate::{context::AuthSignal, error::Result, requests::req_no_body};
use api::routes::leaderboard::*;
use reqwasm::http::Method;

/// GET /api/leaderboard
pub async fn overall_leaderboard(count: usize, offset: usize) -> Result<LeaderboardResponse> {
    req_no_body(
        &format!("/leaderboard?count={count}&offset={offset}"),
        Method::GET,
        None,
    )
    .await
}

/// GET /api/leaderboard/friends
pub async fn friends_leaderboard(auth_signal: &AuthSignal) -> Result<LeaderboardResponse> {
    req_no_body("/leaderboard/friends", Method::GET, Some(auth_signal)).await
}
