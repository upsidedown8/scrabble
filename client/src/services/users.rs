//! Convenience methods for the users api route.

use crate::{
    contexts::AuthSignal,
    error::Result,
    services::{request, std_request, std_request_no_body},
};
use api::{auth::Auth, routes::users::*};
use reqwasm::http::Method;

/// POST /api/users/login
pub async fn login(req: &Login) -> Result<(Auth, UserDetails)> {
    let (auth, user_details) = request(None, "/users/login", req, Method::POST).await?;
    Ok((auth.unwrap(), user_details))
}

/// POST /api/users
pub async fn sign_up(req: &SignUp) -> Result<(Auth, UserDetails)> {
    let (auth, user_details) = request(None, "/users", req, Method::POST).await?;
    Ok((auth.unwrap(), user_details))
}

/// GET /api/users [+Auth]
pub async fn profile(auth_ctx: &AuthSignal) -> Result<UserDetails> {
    std_request_no_body(auth_ctx, "/users", Method::GET).await
}

/// PUT /api/users [+Auth]
pub async fn update(auth_ctx: &AuthSignal, req: &UpdateAccount) -> Result<()> {
    std_request(auth_ctx, "/users", req, Method::PUT).await
}

/// DELETE /api/users [+Auth]
pub async fn delete(auth_ctx: &AuthSignal, req: &DeleteAccount) -> Result<()> {
    std_request(auth_ctx, "/users", req, Method::DELETE).await
}

/// POST /api/users/reset-password
pub async fn reset_password(req: &ResetPassword) -> Result<()> {
    let (_, ()) = request(None, "/users/reset-password", req, Method::POST).await?;

    // no value is returned from this route.
    Ok(())
}

/// GET /api/users/reset-password
pub async fn reset_with_secret(req: &ResetWithSecret) -> Result<(Auth, UserDetails)> {
    let (auth, user_details) = request(None, "/users/reset-password", req, Method::GET).await?;
    Ok((auth.unwrap(), user_details))
}
