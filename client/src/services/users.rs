//! Convenience methods for the users api route.

use crate::{contexts::AuthSignal, error::Error, services::make_request};
use api::routes::users::*;
use reqwasm::http::Method;

/// POST /api/users/login
pub async fn login(auth_ctx: &AuthSignal, req: &Login) -> Result<LoginResponse, Error> {
    make_request(auth_ctx, "/users/login", req, Method::POST)
        .await?
        .1
}

/// POST /api/users
pub async fn sign_up(auth_ctx: &AuthSignal, req: &SignUp) -> Result<SignUpResponse, Error> {
    make_request(auth_ctx, "/users", req, Method::POST).await?.1
}

/// GET /api/users [+Auth]
pub async fn profile(auth_ctx: &AuthSignal) -> Result<ProfileResponse, Error> {
    make_request(auth_ctx, "/users", &(), Method::GET).await?.1
}

/// PUT /api/users [+Auth]
pub async fn update(auth_ctx: &AuthSignal, req: &UpdateAccount) -> Result<(), Error> {
    make_request(auth_ctx, "/users", req, Method::PUT).await?.1
}

/// DELETE /api/users [+Auth]
pub async fn delete(auth_ctx: &AuthSignal, req: &DeleteAccount) -> Result<(), Error> {
    make_request(auth_ctx, "/users", req, Method::DELETE)
        .await?
        .1
}
