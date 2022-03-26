use argon2::{self, Config};
use rand::Rng;

use crate::error::{Error, Result};

// Only need to be initialized once
lazy_static::lazy_static! {
    static ref CONFIG: Config<'static> = Config::default();
}

/// Generates a random salt and hashes the password bytes.
pub fn hash(pwd: &str) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();

    argon2::hash_encoded(pwd.as_bytes(), &salt, &CONFIG).unwrap()
}

/// Verifies that a password matches the stored hash.
pub fn verify(encoded_hash: &str, pwd: &str) -> Result<()> {
    let verified = argon2::verify_encoded(encoded_hash, pwd.as_bytes()).map_err(Error::Argon2)?;

    match verified {
        true => Ok(()),
        false => Err(Error::IncorrectPassword),
    }
}
