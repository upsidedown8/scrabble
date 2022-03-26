//! Provides methods that use Regex to validate usernames,
//! emails and passwords.

use crate::error::{Error, Result};
use regex::Regex;

/// Checks that the username only contains valid characters.
///
/// Allowed characters are:
///     "a-zA-Z0-9_-" or a space
///
/// Minimum length is 3, maximum length is 20.
/// Usernames cannot contain a space, underscore or dash at
/// the start or end.
pub fn validate_username(username: &str) -> Result<()> {
    // use lazy static to ensure the regular expression is only
    // compiled once.
    lazy_static::lazy_static! {
        static ref USERNAME_RGX: Regex = Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9_\- ]{1,18}[a-zA-Z0-9]$").unwrap();
    };

    match USERNAME_RGX.is_match(username) {
        true => Ok(()),
        false => Err(Error::InvalidUsername),
    }
}

/// Checks that the password has sufficient complexity.
///
/// Allowed characters: any
/// Length: at least 8 characters and no more than 50.
/// Required characters:
///     * 1 or more of: {any symbol}
///     * 1 or more of: 0-9
///     * 1 or more of: a-z
///     * 1 or more of: A-Z
pub fn validate_password_complexity(password: &str) -> Result<()> {
    let mut upper = 0;
    let mut lower = 0;
    let mut numbers = 0;
    let mut symbols = 0;

    for ch in password.chars() {
        match ch {
            'a'..='z' => lower += 1,
            'A'..='Z' => upper += 1,
            '0'..='9' => numbers += 1,
            _ => symbols += 1,
        }
    }

    if upper == 0 || lower == 0 || numbers == 0 || symbols == 0 {
        Err(Error::InvalidPassword)
    } else {
        Ok(())
    }
}

/// Checks that the email is valid.
///
/// Allowed emails are of the form {name}@{host}.
/// where {name} contains at least one of [\._-a-zA-Z0-9],
/// and {host} is of the form {domain name}[.{tld}]+. (This
/// also allows for subdomains).
pub fn validate_email(email: &str) -> Result<()> {
    lazy_static::lazy_static! {
        static ref EMAIL_RGX: Regex = Regex::new(r"^[a-zA-Z0-9][\._\-a-zA-Z0-9]+@[a-zA-Z0-9][-_a-zA-Z0-9]*(\.[a-zA-Z]+)+$").unwrap();
        //                                            {name[0]}  {name[....]}   @ {host[0]}  {host[....]}  [.{subdomain}]+
    };

    match EMAIL_RGX.is_match(email) {
        true => Ok(()),
        false => Err(Error::InvalidEmail),
    }
}
