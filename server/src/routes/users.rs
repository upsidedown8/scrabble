use crate::auth::{self, AuthenticatedUser};
use crate::db::user::DbUser;

use crate::{routes::Response, AppState};
use api::users::*;
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
        let id = DbUser::insert(&req.username, &req.email, &hash, &state.pool)
            .await
            .ok_or(Status::InternalServerError)?;

        Ok(Json::from(UserCreateResponse {
            auth: auth::generate_token(&req.username, id).ok_or(Status::InternalServerError)?,
            user_details: UserDetails {
                username: req.username.clone(),
                email: req.email.clone(),
            },
        }))
    }
}

#[post("/users/login", data = "<req>", format = "json")]
pub async fn login(
    state: &State<AppState<'_>>,
    req: Json<UserLogin>,
) -> Response<UserLoginResponse> {
    log::info!("login user: {}", req.username);

    let db_user = DbUser::find_by_username(req.username.trim(), &state.pool)
        .await
        .map_err(|_| Status::Unauthorized)?;
    match auth::verify(&db_user.hashed_pass, req.password.as_bytes()) {
        true => Response::Ok(Json::from(UserLoginResponse {
            auth: db_user.gen_auth()?,
            user_details: UserDetails {
                username: db_user.username,
                email: db_user.email,
            },
        })),
        false => Response::Err(Status::Unauthorized),
    }
}

#[get("/user")]
pub async fn get_details(
    state: &State<AppState<'_>>,
    user: AuthenticatedUser,
) -> Response<UserInfoResponse> {
    log::info!("details for user: {}", user.username);

    match DbUser::find_by_id(&user.id_user, &state.pool).await {
        Ok(db_user) => Response::Ok(Json::from(UserInfoResponse {
            auth: user.regen_auth()?,
            user_details: UserDetails {
                username: db_user.username,
                email: db_user.email,
            },
        })),
        Err(_) => Response::Err(Status::NotFound),
    }
}

#[put("/user", data = "<req>", format = "json")]
pub async fn update(
    state: &State<AppState<'_>>,
    req: Json<UserUpdate>,
    user: AuthenticatedUser,
) -> Response<UserUpdateResponse> {
    log::info!("update user: {}", user);

    match DbUser::find_by_id(&user.id_user, &state.pool).await {
        Ok(db_user) => {
            // check that old password matches
            if !auth::verify(&db_user.hashed_pass, req.old_password.as_bytes()) {
                return Err(Status::Unauthorized);
            }

            let updated_user = DbUser {
                id_user: db_user.id_user,
                username: req
                    .username
                    .clone()
                    .unwrap_or_else(|| db_user.username.clone()),
                email: req.email.clone().unwrap_or_else(|| db_user.email.clone()),
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
                auth: user.regen_auth()?,
                user_details: UserDetails {
                    username: db_user.username,
                    email: db_user.email,
                },
            }))
        }
        Err(_) => Err(Status::NotFound),
    }
}

#[delete("/user", data = "<req>", format = "json")]
pub async fn delete(
    state: &State<AppState<'_>>,
    req: Json<DeleteUser>,
    user: AuthenticatedUser,
) -> Response<DeleteUserResponse> {
    log::info!("delete user: {}", user.username);

    match DbUser::find_by_id(&user.id_user, &state.pool).await {
        Ok(db_user) => {
            // check password to confirm delete
            if !auth::verify(&db_user.hashed_pass, req.password.as_bytes()) {
                return Err(Status::Unauthorized);
            }

            // delete user
            db_user
                .delete(&state.pool)
                .await
                .map_err(|_| Status::InternalServerError)?;

            // return user info
            Ok(Json::from(DeleteUserResponse {
                user_details: UserDetails {
                    username: db_user.username,
                    email: db_user.email,
                },
            }))
        }
        Err(_) => Err(Status::InternalServerError),
    }
}
