use super::make_request;
use api::users::*;
use reqwest::Method;

pub async fn login(req: &Login) -> anyhow::Result<LoginResponse> {
    make_request("/users/login", req, Method::POST).await
}

pub async fn sign_up(req: &SignUp) -> anyhow::Result<SignUpResponse> {
    make_request("/users", req, Method::POST).await
}

pub async fn delete(req: &DeleteAccount) -> anyhow::Result<()> {
    make_request("/users", req, Method::DELETE).await
}

pub async fn update(req: &UpdateAccount) -> anyhow::Result<UpdateAccountResponse> {
    make_request("/users", req, Method::PUT).await
}

pub async fn get() -> anyhow::Result<ProfileResponse> {
    make_request("/users", &(), Method::GET).await
}
