use crate::{
    auth::{self, authenticated_user, hex, validation, Jwt, Role},
    error::Error,
    models, with_db, with_mailer, Db, Mailer,
};
use api::{
    auth::AuthWrapper,
    routes::users::{
        DeleteAccount, Login, LoginResponse, ProfileResponse, ResetPassword,
        ResetPasswordWithSecret, SignUp, SignUpResponse, UpdateAccount, UserDetails,
    },
};
use chrono::{Duration, Utc};
use rand::Rng;
use warp::{Filter, Rejection, Reply};

lazy_static::lazy_static! {
    /// Duration of the password timeout. A password reset link is only
    /// valid for this time, and before it has elapsed a new link cannot
    /// be generated.
    static ref PASSWORD_TIMEOUT: Duration = Duration::seconds(60);
}

/// Filters for the users routes.
pub fn all(
    db: &Db,
    mailer: &Mailer,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let login_route = warp::path("login")
        .and(warp::post())
        .and(with_db(db))
        .and(warp::body::json())
        .and_then(login);
    let sign_up_route = warp::any()
        .and(warp::post())
        .and(with_db(db))
        .and(warp::body::json())
        .and_then(sign_up);
    let profile_route = warp::any()
        .and(warp::get())
        .and(with_db(db))
        .and(authenticated_user())
        .and_then(profile);
    let delete_route = warp::any()
        .and(warp::delete())
        .and(with_db(db))
        .and(authenticated_user())
        .and(warp::body::json())
        .and_then(delete);
    let update_route = warp::any()
        .and(warp::put())
        .and(with_db(db))
        .and(authenticated_user())
        .and(warp::body::json())
        .and_then(update);
    let reset_password_route = warp::path("reset-password")
        .and(warp::post())
        .and(with_db(db))
        .and(with_mailer(mailer))
        .and(warp::body::json())
        .and_then(reset_password);
    let reset_password_with_secret_route = warp::path("reset-password")
        .and(warp::get())
        .and(with_db(db))
        .and(warp::body::json())
        .and_then(reset_password_with_secret);

    let routes = login_route
        .or(sign_up_route)
        .or(profile_route)
        .or(delete_route)
        .or(update_route)
        .or(reset_password_route)
        .or(reset_password_with_secret_route);

    warp::path("users").and(routes)
}

/// POST /api/users/reset-password
async fn reset_password(
    db: Db,
    mailer: Mailer,
    reset_password: ResetPassword,
) -> Result<impl Reply, Rejection> {
    // Find the user specified in the request.
    let user = match reset_password {
        ResetPassword::Email(email) => models::User::find_by_email(&db, &email).await,
        ResetPassword::Username(username) => models::User::find_by_username(&db, &username).await,
    }?;

    // check whether there is an existing record in `tbl_password_reset`.
    if let Ok(pwd_reset) = models::PasswordReset::find_by_id_user(&db, user.id_user()).await {
        // if there is already a password reset entry that
        // has not expired, do not send another.
        if !pwd_reset.is_expired() {
            return Err(Error::ResetTimeout.into());
        }
    }

    // generate a 32 byte secret and convert to hex.
    let secret: [u8; 32] = rand::thread_rng().gen();
    let secret_hex = hex::encode(&secret);
    // add a password reset record to the database.
    let password_reset = models::PasswordReset {
        id_user: user.id_user(),
        secret_hex: secret_hex.clone(),
        valid_until: Utc::now().naive_utc() + *PASSWORD_TIMEOUT,
    };
    password_reset.insert(&db).await?;

    // send a reset password email.
    let body = format!(
        r#"
    <style>
        html {{
            text-align: center;
            margin: 20%;
            font-family: monospace;
        }}
    </style>
    <h1>Password reset</h1>
    <p>
        You are receiving this email because a request was made to
        reset the password for an account with username: {username}.

        <a href="https://scrabble.thrgd.uk/reset-password&secret={hex}&username={username}">
            Click here to reset your password.
        </a>
    </p>
    "#,
        username = user.username,
        hex = secret_hex,
    );
    mailer
        .send(&user.email, "Scrabble AI: Password Reset", body)
        .await?;

    // send a 200 OK reply if the operation succeeded.
    Ok(warp::reply::json(&AuthWrapper {
        auth: None,
        response: (),
    }))
}

/// GET /api/users/reset-password
async fn reset_password_with_secret(
    db: Db,
    with_secret: ResetPasswordWithSecret,
) -> Result<impl Reply, Rejection> {
    // lookup the secret in the database.
    let pwd_reset = models::PasswordReset::find_by_username(&db, &with_secret.username).await?;

    // check that the record has not expired
    if pwd_reset.is_expired() {
        return Err(Error::ResetTimeout.into());
    }

    // compare the secret with the stored value.
    if !pwd_reset.secret_matches(&with_secret.secret_hex) {
        return Err(Error::IncorrectResetSecret.into());
    }

    // validate the provided password.
    validation::validate_password_complexity(&with_secret.new_password)?;
    let new_hashed_pass = auth::hash(&with_secret.new_password);

    // update the user's password.
    let user = models::User::find_by_id(&db, pwd_reset.id_user).await?;
    let new_user = models::User {
        hashed_pass: new_hashed_pass,
        ..user
    };
    new_user.update(&db).await?;

    // delete the reset password record
    pwd_reset.delete(&db).await?;

    // generate a jwt.
    let jwt = Jwt::new(new_user.id_user(), new_user.role());

    Ok(warp::reply::json(&AuthWrapper {
        auth: Some(jwt.auth()?),
        response: (),
    }))
}

/// POST /api/users/login
async fn login(db: Db, login: Login) -> Result<impl Reply, Rejection> {
    let user = models::User::find_by_username(&db, login.username.trim()).await?;
    let jwt = Jwt::new(user.id_user(), user.role());

    auth::verify(&user.hashed_pass, &login.password)?;

    Ok(warp::reply::json(&AuthWrapper {
        auth: Some(jwt.auth()?),
        response: LoginResponse {
            user_details: user.into_user_details(),
        },
    }))
}

/// POST /api/users
async fn sign_up(db: Db, sign_up: SignUp) -> Result<impl Reply, Rejection> {
    validation::validate_password_complexity(&sign_up.password)?;
    validation::validate_username(&sign_up.username)?;
    validation::validate_email(&sign_up.email)?;
    models::User::check_username_free(&db, &sign_up.username).await?;

    let hashed_pass = auth::hash(&sign_up.password);
    let id_user = models::User::insert(
        &db,
        &sign_up.username,
        &sign_up.email,
        &hashed_pass,
        Role::User,
        sign_up.is_private,
    )
    .await?;

    let jwt = Jwt::new(id_user, Role::User);

    Ok(warp::reply::json(&AuthWrapper {
        auth: Some(jwt.auth()?),
        response: SignUpResponse {
            user_details: UserDetails {
                username: sign_up.username,
                email: sign_up.email,
                is_private: sign_up.is_private,
            },
        },
    }))
}

/// GET /api/users [+Auth]
async fn profile(db: Db, jwt: Jwt) -> Result<impl Reply, Rejection> {
    let user = models::User::find_by_id(&db, jwt.id_user()).await?;

    Ok(warp::reply::json(&AuthWrapper {
        auth: Some(jwt.auth()?),
        response: ProfileResponse {
            user_details: user.into_user_details(),
        },
    }))
}

/// PUT /api/users [+Auth]
async fn update(db: Db, jwt: Jwt, update: UpdateAccount) -> Result<impl Reply, Rejection> {
    let user = models::User::find_by_id(&db, jwt.id_user()).await?;
    auth::verify(&user.hashed_pass, &update.old_password)?;

    let updated_user = models::User {
        username: update.username.unwrap_or_else(|| user.username.clone()),
        email: update.email.unwrap_or_else(|| user.email.clone()),
        hashed_pass: update
            .password
            .as_deref()
            .map(auth::hash)
            .unwrap_or_else(|| user.hashed_pass.clone()),
        is_private: update.is_private.unwrap_or(user.is_private),
        date_updated: Utc::now().naive_utc(),
        ..user.clone()
    };

    // ensure that the new username, email and password are still valid.
    if let Some(pwd) = update.password.as_deref() {
        validation::validate_password_complexity(pwd)?;
    }
    validation::validate_username(&updated_user.username)?;
    validation::validate_email(&updated_user.email)?;

    updated_user.update(&db).await?;

    Ok(warp::reply::json(&AuthWrapper {
        auth: Some(jwt.auth()?),
        response: (),
    }))
}

/// DELETE /api/users [+Auth]
async fn delete(db: Db, jwt: Jwt, delete: DeleteAccount) -> Result<impl Reply, Rejection> {
    let user = models::User::find_by_id(&db, jwt.id_user()).await?;
    auth::verify(&user.hashed_pass, &delete.password)?;
    user.delete(&db).await?;

    Ok(warp::reply::json(&AuthWrapper {
        auth: None,
        response: (),
    }))
}
