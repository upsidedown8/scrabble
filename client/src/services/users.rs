use crate::error::Error;

use super::make_request;
use crate::contexts::auth::AuthSignal;
use api::users::*;
use reqwasm::http::Method;

pub async fn login(auth_ctx: &AuthSignal, req: &Login) -> Result<LoginResponse, Error> {
    make_request(auth_ctx, "/users/login", req, Method::POST).await
}

pub async fn sign_up(auth_ctx: &AuthSignal, req: &SignUp) -> Result<SignUpResponse, Error> {
    make_request(auth_ctx, "/users", req, Method::POST).await
}

pub async fn profile(auth_ctx: &AuthSignal) -> Result<ProfileResponse, Error> {
    make_request(auth_ctx, "/users", &(), Method::GET).await
}

pub async fn update(auth_ctx: &AuthSignal, req: &UpdateAccount) -> Result<UpdateAccountResponse, Error> {
    make_request(auth_ctx, "/users", req, Method::PUT).await
}

pub async fn delete(auth_ctx: &AuthSignal, req: &DeleteAccount) -> Result<(), Error> {
    make_request(auth_ctx, "/users", req, Method::DELETE).await
}
