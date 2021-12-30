use argon2::Config;
use chrono::{Duration, Utc};
use common::api::users::Auth;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use rand::Rng;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};
use serde::{Deserialize, Serialize};
use std::{env, fmt};
use uuid::Uuid;

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

lazy_static! {
    static ref JWT_SECRET: Vec<u8> = {
        let jwt_secret =
            env::var("JWT_SECRET").expect("expected `JWT_SECRET` environment variable");
        hex::decode(&jwt_secret).expect("`JWT_SECRET` should be valid hex")
    };
    static ref ENCODING_KEY: EncodingKey = EncodingKey::from_secret(&JWT_SECRET);
    static ref DECODING_KEY: DecodingKey<'static> = DecodingKey::from_secret(&JWT_SECRET);
    static ref VALIDATION: Validation = Validation::default();
}

/// Expiry time in seconds
const JWT_EXPIRY: i64 = 1800;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// User id
    pub id_user: Uuid,
    /// Expiry time
    pub exp: usize,
    /// Subject (username)
    pub sub: String,
}

/// Generate a JWT for a user.
pub fn generate_token(username: &str, id_user: Uuid) -> Option<Auth> {
    // get current time, and add `jwt_expiry` to get final time
    let exp = Utc::now()
        .checked_add_signed(Duration::seconds(JWT_EXPIRY))
        .expect("no overflow")
        .timestamp() as usize;

    let claims = Claims {
        exp,
        id_user,
        sub: username.to_string(),
    };

    encode(&Header::default(), &claims, &ENCODING_KEY)
        .map(Auth)
        .ok()
}

/// Used as a request guard for handlers that require login
pub struct AuthenticatedUser {
    pub id_user: Uuid,
    pub username: String,
}

#[derive(Debug)]
pub enum LoginError {
    MissingHeader,
    IncorrectDetails,
}

impl fmt::Display for AuthenticatedUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]: {}", self.id_user, self.username)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = LoginError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_token = request
            .headers()
            .get("Authorization")
            .next()
            .map(|h| h.split_whitespace())
            .and_then(|mut h| h.nth(1));

        match auth_token {
            None => Outcome::Failure((Status::Unauthorized, LoginError::MissingHeader)),
            Some(token) => match decode::<Claims>(token, &DECODING_KEY, &VALIDATION) {
                Ok(jwt) => Outcome::Success(Self {
                    id_user: jwt.claims.id_user,
                    username: jwt.claims.sub,
                }),
                Err(_) => Outcome::Failure((Status::Unauthorized, LoginError::IncorrectDetails)),
            },
        }
    }
}
