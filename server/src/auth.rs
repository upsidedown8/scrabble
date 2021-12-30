use argon2::Config;
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rand::Rng;
use serde::{Deserialize, Serialize};

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
pub fn generate_token(
    jwt_secret: &[u8],
    jwt_expiry: usize,
    username: &str,
) -> anyhow::Result<String> {
    // get current time, and add `jwt_expiry` to get final time
    let timestamp = Utc::now().timestamp() as usize;
    let exp = timestamp + jwt_expiry;

    let header = Header::default();
    let claims = Claims {
        exp,
        sub: username.to_string(),
    };
    let key = EncodingKey::from_secret(jwt_secret);

    encode(&header, &claims, &key).map_err(|e| e.into())
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
