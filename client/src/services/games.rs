//! Convenience methods for the games api route.

use crate::{contexts::AuthSignal, error::Result, services::std_request_no_body};
use api::routes::games::*;
use reqwasm::http::Method;

/// GET /api/games [+Auth]
pub async fn list(auth_ctx: &AuthSignal) -> Result<ListGamesResponse> {
    std_request_no_body(auth_ctx, "/games", Method::GET).await
}

/// GET /api/games/{game id}/stats [+Auth]
pub async fn stats(auth_ctx: &AuthSignal, id_game: i32) -> Result<GameStatsResponse> {
    let url = format!("/games/{id_game}/stats");
    std_request_no_body(auth_ctx, &url, Method::GET).await
}

/// GET /api/games/stats [+Auth]
pub async fn overall_stats(auth_ctx: &AuthSignal) -> Result<OverallStatsResponse> {
    std_request_no_body(auth_ctx, "/games/stats", Method::GET).await
}
