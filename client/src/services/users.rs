use super::make_request;
use api::users::*;
use reqwest::Method;

pub async fn login(req: &UserLogin) -> anyhow::Result<UserLoginResponse> {
    make_request("/users/login", req, Method::POST).await
}

pub async fn create(req: &UserCreate) -> anyhow::Result<UserCreateResponse> {
    make_request("/users", req, Method::POST).await
}

pub async fn delete(req: &DeleteUser) -> anyhow::Result<DeleteUserResponse> {
    make_request("/user", req, Method::DELETE).await
}

pub async fn update(req: &UserUpdate) -> anyhow::Result<UserUpdateResponse> {
    make_request("/user", req, Method::PUT).await
}

pub async fn get() -> anyhow::Result<UserInfoResponse> {
    make_request("/user", &(), Method::GET).await
}
