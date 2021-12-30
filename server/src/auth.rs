use argon2::Config;
use chrono::Utc;
use common::api::users::Auth;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rand::Rng;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request, State,
};
use serde::{Deserialize, Serialize};

use crate::{auth, db::user::DbUser, AppState};

/// Generates a random salt and hashes the password.
pub fn hash(pass: &[u8]) -> anyhow::Result<String> {
    let salt: [u8; 32] = rand::thread_rng().gen();
    let cfg = Config::default();
    argon2::hash_encoded(pass, &salt, &cfg).map_err(|e| e.into())
}

/// Checks that `pass` hashes to `hash` (salt stored in `hash`).
pub fn verify(hash: &str, pass: &[u8]) -> bool {
    argon2::verify_encoded(hash, pass).unwrap_or(false)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Expiry time
    pub exp: usize,
    /// Subject (username)
    pub sub: String,
    /// Ensure the jwt is from the same server session
    pub session: usize,
    /// Store database id to prevent using token from deleted account with
    /// matching username
    pub id: i64,
}

/// Generate a JWT.
pub fn generate_token(state: &State<AppState>, username: &str, id: i64) -> Option<Auth> {
    // get current time, and add `jwt_expiry` to get final time
    let timestamp = Utc::now().timestamp() as usize;
    let exp = timestamp + state.jwt.jwt_expiry_delta;

    let header = Header::default();
    let claims = Claims {
        exp,
        id,
        sub: username.to_string(),
        session: state.jwt.jwt_session,
    };
    let key = EncodingKey::from_secret(&state.jwt.jwt_secret);

    encode(&header, &claims, &key).map(Auth).ok()
}

/// Check whether a user is authorised, provided their token.
pub fn validate_token(state: &State<AppState>, username: &str, id: i64, token: &str) -> bool {
    log::info!("{}", username);

    let validation = Validation {
        sub: Some(username.to_string()),
        ..Validation::default()
    };

    let key = DecodingKey::from_secret(&state.jwt.jwt_secret);

    log::info!("{:?}", decode::<Claims>(token, &key, &validation));

    decode::<Claims>(token, &key, &validation)
        .map(|t| t.claims.session == state.jwt.jwt_session && t.claims.id == id)
        .unwrap_or(false)
}

/// Used as a request guard for handlers that require login
pub struct ApiKey<'r>(&'r str);

#[derive(Debug)]
pub enum ApiKeyError {
    Missing,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = ApiKeyError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get("Authorization").next();

        let auth_token = auth_header
            .map(|h| h.split_whitespace())
            .and_then(|mut h| h.nth(1));

        match auth_token {
            Some(token) => Outcome::Success(Self(token)),
            None => Outcome::Failure((Status::Unauthorized, ApiKeyError::Missing)),
        }
    }
}

impl<'r> ApiKey<'r> {
    pub async fn verify(&self, username: &str, state: &State<AppState<'_>>) -> Result<(), Status> {
        let id = DbUser::find_id(username, &state.pool)
            .await
            .map_err(|_| Status::Unauthorized)?;

        match auth::validate_token(state, username, id, self.0) {
            true => Ok(()),
            false => Err(Status::Unauthorized),
        }
    }
}
