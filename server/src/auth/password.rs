use argon2::{self, Config};
use rand::Rng;

// Only need to be initialized once
lazy_static::lazy_static! {
    static ref CONFIG: Config<'static> = Config::default();
}

/// Generates a random salt and hashes the password bytes.
pub fn hash(pwd: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();

    argon2::hash_encoded(pwd, &salt, &CONFIG).unwrap()
}

/// Verifies that a password matches the stored hash.
pub fn verify(encoded_hash: &str, pwd: &[u8]) -> bool {
    argon2::verify_encoded(encoded_hash, pwd).unwrap_or(false)
}

// pub struct PasswordStrength;

// /// Determines the strength of a password.
// pub fn strength(pwd: &[u8]) -> PasswordStrength {
//     todo!();
// }
