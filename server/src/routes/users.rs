use crate::auth;
use crate::db::user::DbUser;

use crate::{routes::Response, AppState};
use common::api::users::*;
use rocket::{http::Status, serde::json::Json, State};

#[post("/users", data = "<req>", format = "json")]
pub async fn create(
    state: &State<AppState<'_>>,
    req: Json<UserCreate>,
) -> Response<UserCreateResponse> {
    log::info!("create user");

    todo!()
}

#[post("/users/<username>", data = "<req>", format = "json")]
pub async fn login(
    state: &State<AppState<'_>>,
    username: String,
    req: Json<UserLogin>,
) -> Response<UserLoginResponse> {
    log::info!("login user: {}", username);

    let user = DbUser::find_by_username(username.trim(), &state.pool)
        .await
        .map_err(|_| Status::Unauthorized)?;
    match auth::verify(&user.hashed_pass, req.password.as_bytes()) {
        true => Response::Ok(Json::from(UserLoginResponse {
            auth: Auth {
                token: auth::generate_token(&state.jwt_secret, state.jwt_expiry, &user.username)
                    .map_err(|_| Status::Unauthorized)?,
            },
            user_details: UserDetails {
                username: user.username,
                email: user.email,
            },
        })),
        false => Response::Err(Status::Unauthorized),
    }
}

#[get("/users/<username>", data = "<req>", format = "json")]
pub async fn get_details(
    state: &State<AppState<'_>>,
    username: String,
    req: Json<UserInfo>,
) -> Response<UserInfoResponse> {
    log::info!("details for user: {}", username);

    if !auth::validate_token(&state.jwt_secret, &username, &req.auth.token) {
        return Response::Err(Status::Unauthorized);
    }

    match DbUser::find_by_username(&username, &state.pool).await {
        Ok(user) => Response::Ok(Json::from(UserInfoResponse {
            user_details: UserDetails {
                username: user.username,
                email: user.email,
            },
        })),
        Err(_) => Response::Err(Status::NotFound),
    }
}

#[put("/users/<username>", data = "<req>", format = "json")]
pub async fn update(
    state: &State<AppState<'_>>,
    username: String,
    req: Json<UserUpdate>,
) -> Response<UserUpdateResponse> {
    log::info!("update user: {}", username);

    todo!()
}

#[delete("/users/<username>", data = "<req>", format = "json")]
pub async fn delete(
    state: &State<AppState<'_>>,
    username: String,
    req: Json<DeleteUser>,
) -> Response<DeleteUserResponse> {
    log::info!("delete user: {}", username);

    todo!()
}
