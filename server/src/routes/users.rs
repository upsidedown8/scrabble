use crate::auth::{self, ApiKey};
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

    // if exists, cannot create
    if DbUser::find_by_username(&req.username, &state.pool)
        .await
        .is_ok()
    {
        Err(Status::InternalServerError)
    } else {
        let hash = auth::hash(req.password.as_bytes()).map_err(|_| Status::InternalServerError)?;
        DbUser::insert(&req.username, &req.email, &hash, &state.pool)
            .await
            .map_err(|_| Status::InternalServerError)?;

        Ok(Json::from(UserCreateResponse {
            auth: auth::generate_token(state, &req.username).ok_or(Status::InternalServerError)?,
            user_details: UserDetails {
                username: req.username.clone(),
                email: req.email.clone(),
            },
        }))
    }
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
            auth: auth::generate_token(state, &user.username).ok_or(Status::Unauthorized)?,
            user_details: UserDetails {
                username: user.username,
                email: user.email,
            },
        })),
        false => Response::Err(Status::Unauthorized),
    }
}

#[get("/users/<username>")]
pub async fn get_details(
    state: &State<AppState<'_>>,
    username: String,
    key: ApiKey<'_>,
) -> Response<UserInfoResponse> {
    log::info!("details for user: {}", username);

    key.verify(&username, state).await?;

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
    key: ApiKey<'_>,
) -> Response<UserUpdateResponse> {
    log::info!("update user: {}", username);

    key.verify(&username, state).await?;

    match DbUser::find_by_username(&username, &state.pool).await {
        Ok(user) => {
            // check that old password matches
            if !auth::verify(&user.hashed_pass, req.old_password.as_bytes()) {
                return Err(Status::Unauthorized);
            }

            let updated_user = DbUser {
                id_user: user.id_user,
                username: req.username.clone().unwrap_or_else(|| username.clone()),
                email: req.email.clone().unwrap_or_else(|| user.email.clone()),
                hashed_pass: auth::hash(
                    req.password
                        .as_ref()
                        .unwrap_or(&req.old_password)
                        .as_bytes(),
                )
                .map_err(|_| Status::InternalServerError)?,
            };

            updated_user
                .update(&state.pool)
                .await
                .map_err(|_| Status::InternalServerError)?;

            Ok(Json::from(UserUpdateResponse {
                user_details: UserDetails {
                    username,
                    email: user.email,
                },
            }))
        }
        Err(_) => Err(Status::NotFound),
    }
}

#[delete("/users/<username>", data = "<req>", format = "json")]
pub async fn delete(
    state: &State<AppState<'_>>,
    username: String,
    req: Json<DeleteUser>,
    key: ApiKey<'_>,
) -> Response<DeleteUserResponse> {
    log::info!("delete user: {}", username);

    key.verify(&username, state).await?;

    match DbUser::find_by_username(&username, &state.pool).await {
        Ok(user) => {
            // check password to confirm delete
            if !auth::verify(&user.hashed_pass, req.password.as_bytes()) {
                return Err(Status::Unauthorized);
            }

            // delete user
            user.delete(&state.pool)
                .await
                .map_err(|_| Status::InternalServerError)?;

            // return user info
            Ok(Json::from(DeleteUserResponse {
                user_details: UserDetails {
                    username,
                    email: user.email,
                },
            }))
        }
        Err(_) => Err(Status::InternalServerError),
    }
}
