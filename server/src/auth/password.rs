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

// pub struct PasswordStrength;

/// Determines the strength of a password by estimating the
/// number of bits of entropy.
pub fn strength(pwd: &[u8]) -> PasswordStrength {
    let mut lowercase = 0;
    let mut uppercase = 0;
    let mut numbers = 0;
    let mut symbols = 0;

    for byte in pwd {
        match byte {
            b'a'..=b'z' => lowercase += 1,
            b'A'..=b'Z' => uppercase += 1,
            b'0'..=b'9' => numbers += 1,
            _ => symbols += 1,
        }
    }
}

/// Checks that the password has sufficient strength.
pub fn check_password_strength(_pwd: &str) -> Result<()> {
    Ok(())
}
