use crate::error::Error;

use super::make_request;
use api::users::*;
use reqwasm::http::Method;
use sycamore::prelude::ScopeRef;

pub async fn login(ctx: ScopeRef<'_>, req: &Login) -> Result<LoginResponse, Error> {
    make_request(ctx, "/users/login", req, Method::POST).await
}

pub async fn sign_up(ctx: ScopeRef<'_>, req: &SignUp) -> Result<SignUpResponse, Error> {
    make_request(ctx, "/users", req, Method::POST).await
}

pub async fn profile(ctx: ScopeRef<'_>) -> Result<ProfileResponse, Error> {
    make_request(ctx, "/users", &(), Method::GET).await
}

pub async fn update(ctx: ScopeRef<'_>, req: &UpdateAccount) -> Result<UpdateAccountResponse, Error> {
    make_request(ctx, "/users", req, Method::PUT).await
}

pub async fn delete(ctx: ScopeRef<'_>, req: &DeleteAccount) -> Result<(), Error> {
    make_request(ctx, "/users", req, Method::DELETE).await
}
