//! Convenience methods for the games api route.

use crate::{context::AuthSignal, error::Result, requests::req_no_body};
use api::routes::games::*;
use reqwasm::http::Method;

/// GET /api/games [+Auth]
pub async fn list(auth_signal: &AuthSignal) -> Result<ListGamesResponse> {
    req_no_body("/games", Method::GET, Some(auth_signal)).await
}

/// GET /api/games/{game id}/stats [+Auth]
pub async fn stats(auth_signal: &AuthSignal, id_game: i32) -> Result<GameStatsResponse> {
    req_no_body(
        &format!("/games/{id_game}/stats"),
        Method::GET,
        Some(auth_signal),
    )
    .await
}

/// GET /api/games/stats [+Auth]
pub async fn overall_stats(auth_signal: &AuthSignal) -> Result<OverallStatsResponse> {
    req_no_body("/games/stats", Method::GET, Some(auth_signal)).await
}
