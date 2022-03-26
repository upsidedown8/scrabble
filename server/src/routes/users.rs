use crate::{
    auth::{self, authenticated_user, Jwt, Role},
    models::{user::User, with_db, Db},
};
use api::users::{
    DeleteAccount, Login, LoginResponse, ProfileResponse, SignUp, SignUpResponse, UpdateAccount,
    UpdateAccountResponse,
};
use uuid::Uuid;
use warp::{Filter, Rejection, Reply};

/// Filters for the users routes.
pub fn all(db: Db) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let login_route = warp::path("login")
        .and(warp::post())
        .and(with_db(db.clone()))
        .and(warp::body::json())
        .and_then(login);
    let sign_up_route = warp::any()
        .and(warp::post())
        .and(with_db(db.clone()))
        .and(warp::body::json())
        .and_then(sign_up);
    let profile_route = warp::any()
        .and(warp::get())
        .and(with_db(db.clone()))
        .and(authenticated_user())
        .and_then(profile);
    let delete_route = warp::any()
        .and(warp::delete())
        .and(with_db(db.clone()))
        .and(authenticated_user())
        .and(warp::body::json())
        .and_then(delete);
    let update_route = warp::any()
        .and(warp::put())
        .and(with_db(db))
        .and(authenticated_user())
        .and(warp::body::json())
        .and_then(update);

    let routes = login_route
        .or(sign_up_route)
        .or(profile_route)
        .or(delete_route)
        .or(update_route);

    warp::path("users").and(routes)
}

/// POST /api/users/login
async fn login(db: Db, login: Login) -> Result<impl Reply, Rejection> {
    let user = User::find_by_username(&db, login.username.trim()).await?;

    auth::verify(&user.hashed_pass, &login.password)?;

    let jwt = Jwt::new(user.id_user()?, user.role());

    Ok(warp::reply::json(&LoginResponse {
        auth: jwt.auth()?,
        user_details: user.into_user_details(),
    }))
}

/// POST /api/users
async fn sign_up(db: Db, sign_up: SignUp) -> Result<impl Reply, Rejection> {
    auth::check_username_valid(&sign_up.username)?;
    auth::check_password_strength(&sign_up.password)?;
    User::check_username_free(&db, &sign_up.username).await?;

    let id_user = Uuid::new_v4();
    let user = User {
        id_user: id_user.to_string(),
        username: sign_up.username,
        email: sign_up.email,
        hashed_pass: auth::hash(&sign_up.password),
        role: Role::User.to_string(),
    };
    let jwt = Jwt::new(id_user, Role::User);

    user.insert(&db).await?;

    Ok(warp::reply::json(&SignUpResponse {
        auth: jwt.auth()?,
        user_details: user.into_user_details(),
    }))
}

/// GET /api/users [+Auth]
async fn profile(db: Db, jwt: Jwt) -> Result<impl Reply, Rejection> {
    let user = User::find_by_id(&db, jwt.id_user()).await?;

    Ok(warp::reply::json(&ProfileResponse {
        auth: jwt.auth()?,
        user_details: user.into_user_details(),
    }))
}

/// PUT /api/users [+Auth]
async fn update(db: Db, jwt: Jwt, update: UpdateAccount) -> Result<impl Reply, Rejection> {
    let user = User::find_by_id(&db, jwt.id_user()).await?;
    auth::verify(&user.hashed_pass, &update.old_password)?;

    let hashed_pass = if let Some(password) = &update.password {
        auth::check_password_strength(password)?;
        auth::hash(password)
    } else {
        user.hashed_pass.clone()
    };

    let updated_user = User {
        username: update.username.unwrap_or_else(|| user.username.clone()),
        email: update.email.unwrap_or_else(|| user.email.clone()),
        hashed_pass,
        ..user.clone()
    };

    updated_user.update(&db).await?;

    Ok(warp::reply::json(&UpdateAccountResponse {
        auth: jwt.auth()?,
    }))
}

/// DELETE /api/users [+Auth]
async fn delete(db: Db, jwt: Jwt, delete: DeleteAccount) -> Result<impl Reply, Rejection> {
    let user = User::find_by_id(&db, jwt.id_user()).await?;
    auth::verify(&user.hashed_pass, &delete.password)?;
    user.delete(&db).await?;

    Ok(warp::reply())
}
