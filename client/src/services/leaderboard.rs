//! Convenience methods for the leaderboard api route.

use crate::{error::Result, services::request_no_body};
use api::routes::leaderboard::*;
use reqwasm::http::Method;

/// GET /api/leaderboard
pub async fn overall_leaderboard(count: usize, offset: usize) -> Result<LeaderboardResponse> {
    let url = format!("/leaderboard?count={count}&offset={offset}");
    let (_, response) = request_no_body(None, &url, Method::GET).await?;
    Ok(response)
}

/// GET /api/leaderboard/friends
pub async fn friends_leaderboard() -> Result<LeaderboardResponse> {
    let (_, response) = request_no_body(None, "/leaderboard/friends", Method::GET).await?;
    Ok(response)
}
