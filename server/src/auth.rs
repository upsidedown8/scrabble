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

use crate::{auth, AppState};

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
}

/// Generate a JWT.
pub fn generate_token(jwt_secret: &[u8], jwt_expiry: usize, username: &str) -> Option<Auth> {
    // get current time, and add `jwt_expiry` to get final time
    let timestamp = Utc::now().timestamp() as usize;
    let exp = timestamp + jwt_expiry;

    let header = Header::default();
    let claims = Claims {
        exp,
        sub: username.to_string(),
    };
    let key = EncodingKey::from_secret(jwt_secret);

    encode(&header, &claims, &key)
        .map(|token| Auth { token })
        .ok()
}

/// Check whether a user is authorised, provided their token.
pub fn validate_token(jwt_secret: &[u8], username: &str, token: &str) -> bool {
    log::info!("{}", username);

    let validation = Validation {
        sub: Some(username.to_string()),
        ..Validation::default()
    };

    let key = DecodingKey::from_secret(jwt_secret);

    log::info!("{:?}", decode::<Claims>(token, &key, &validation));

    decode::<Claims>(token, &key, &validation).is_ok()
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
        match auth::validate_token(&state.jwt_secret, username, self.0) {
            true => Ok(()),
            false => Err(Status::Unauthorized),
        }
    }
}
