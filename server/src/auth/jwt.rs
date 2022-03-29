use crate::{
    auth::hex,
    error::{Error, Result},
};
use api::auth::Auth;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, env, fmt, str::FromStr};

lazy_static::lazy_static! {
    /// Secret used to sign tokens
    static ref JWT_SECRET: Vec<u8> = {
        let jwt_secret = env::var("JWT_SECRET")
            .expect("`JWT_SECRET` env variable to be set");
        let jwt_secret = hex::decode(&jwt_secret);

        if jwt_secret.len() < 32 {
            panic!("`JWT_SECRET` must have at least 32 bytes");
        }

        jwt_secret
    };
    /// Duration for which each token is valid
    static ref JWT_EXPIRY_DURATION: Duration = Duration::seconds(1800);
    static ref ENCODING_KEY: EncodingKey = EncodingKey::from_secret(&JWT_SECRET);
    static ref DECODING_KEY: DecodingKey = DecodingKey::from_secret(&JWT_SECRET);
    static ref VALIDATION: Validation = Validation::default();
    static ref HEADER: Header = Header::default();
}

/// User or admin account.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Role {
    /// Basic access.
    User,
    /// Elevated access.
    Admin,
}

impl FromStr for Role {
    type Err = Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s {
            "admin" => Role::Admin,
            _ => Role::User,
        })
    }
}
impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Role::User => "user",
                Role::Admin => "admin",
            }
        )
    }
}
impl Default for Role {
    fn default() -> Role {
        Role::User
    }
}

/// Jwt claims, encoded in the token.
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    /// Expiry time (after which the token is invalid).
    exp: usize,
    /// Subject (user id).
    id_user: i32,
    /// User role.
    role: Role,
}

/// A (decoded) json web token for a user.
pub struct Jwt(Claims);

impl Jwt {
    /// Creates a new json web token from a user id and role.
    pub fn new(id_user: i32, role: Role) -> Self {
        // exp is set when `to_auth` is called
        Jwt(Claims {
            exp: 0,
            id_user,
            role,
        })
    }
    /// Validates and decodes the JWT.
    pub fn from_auth_token(token: &str, required_role: Role) -> Result<Self> {
        // the decode function also checks that the expiry is valid.
        let jwt = decode::<Claims>(token, &DECODING_KEY, &VALIDATION)
            .map(|token_data| Jwt(token_data.claims))
            .map_err(Error::JwtDecoding)?;
        let has_role = match required_role {
            Role::Admin => jwt.0.role == Role::Admin,
            Role::User => true,
        };

        match has_role {
            false => Err(Error::InsufficientRole),
            true => Ok(jwt),
        }
    }
    /// Gets the `id_user` claims field.
    pub fn id_user(&self) -> i32 {
        self.0.id_user
    }
    /// Encodes the JWT, using the secret and expiry time offset
    /// from the `.env` file.
    pub fn auth(mut self) -> Result<Auth> {
        // get current time, and add `JWT_EXPIRY_SECONDS` to get final time
        let exp_time = Utc::now() + *JWT_EXPIRY_DURATION;
        self.0.exp = exp_time.timestamp() as usize;

        let claims = &self.0;

        encode(&HEADER, claims, &ENCODING_KEY)
            .map(Auth)
            .map_err(Error::JwtEncoding)
    }
}
