//! Convenience methods for the users api route.

use crate::{
    context::AuthSignal,
    error::Result,
    requests::{req_std, request},
};
use api::{auth::Token, routes::users::*};
use reqwasm::http::Method;

/// POST /api/users/login
pub async fn login(req: &Login) -> Result<(Token, UserDetails)> {
    let (token, user_details) = request("/users/login", Method::POST, Some(req), None).await?;
    Ok((token.unwrap(), user_details))
}

/// POST /api/users
pub async fn sign_up(req: &SignUp) -> Result<(Token, UserDetails)> {
    let (token, user_details) = request("/users", Method::POST, Some(req), None).await?;
    Ok((token.unwrap(), user_details))
}

/// PUT /api/users [+Auth]
pub async fn update(auth_signal: &AuthSignal, req: &UpdateAccount) -> Result<()> {
    req_std("/users", Method::PUT, Some(req), Some(auth_signal)).await
}

/// DELETE /api/users [+Auth]
pub async fn delete(auth_signal: &AuthSignal, req: &DeleteAccount) -> Result<()> {
    req_std("/users", Method::DELETE, Some(req), Some(auth_signal)).await
}

/// POST /api/users/reset-password
pub async fn reset_password(req: &ResetPassword) -> Result<()> {
    let (_, ()) = request("/users/reset-password", Method::POST, Some(req), None).await?;

    // no value is returned from this route.
    Ok(())
}

/// PUT /api/users/reset-password
pub async fn reset_with_secret(req: &ResetWithSecret) -> Result<(Token, UserDetails)> {
    let (token, user_details) =
        request("/users/reset-password", Method::GET, Some(req), None).await?;
    Ok((token.unwrap(), user_details))
}
