//! Convenience methods for the users api route.

use crate::{
    contexts::AuthSignal,
    error::Error,
    services::{request, std_request},
};
use api::{auth::Auth, routes::users::*};
use reqwasm::http::Method;

/// POST /api/users/login
pub async fn login(req: &Login) -> Result<(Option<Auth>, LoginResponse), Error> {
    request(None, "/users/login", req, Method::POST).await
}

/// POST /api/users
pub async fn sign_up(
    auth_ctx: &AuthSignal,
    req: &SignUp,
) -> Result<(Option<Auth>, SignUpResponse), Error> {
    request(None, "/users", req, Method::POST).await
}

/// GET /api/users [+Auth]
pub async fn profile(auth_ctx: &AuthSignal) -> Result<ProfileResponse, Error> {
    std_request(auth_ctx, "/users", &(), Method::GET).await
}

/// PUT /api/users [+Auth]
pub async fn update(auth_ctx: &AuthSignal, req: &UpdateAccount) -> Result<(), Error> {
    std_request(auth_ctx, "/users", req, Method::PUT).await
}

/// DELETE /api/users [+Auth]
pub async fn delete(auth_ctx: &AuthSignal, req: &DeleteAccount) -> Result<(), Error> {
    std_request(auth_ctx, "/users", req, Method::DELETE).await
}
