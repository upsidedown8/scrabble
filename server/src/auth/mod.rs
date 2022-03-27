//! Module for performing authorization.

use crate::error::Error;
use warp::{hyper::HeaderMap, Filter, Rejection};

pub mod hex;
pub mod validation;

mod jwt;
mod password;

pub use jwt::{Jwt, Role};
pub use password::{hash, verify};

const AUTHORIZATION: &str = "Authorization";
const BEARER: &str = "Bearer ";

/// Filter which checks whether an auth token is present, valid, and
/// contains the user role.
pub fn authenticated_user() -> impl Filter<Extract = (Jwt,), Error = Rejection> + Copy {
    auth_validation(Role::User)
}

/// Filter which checks whether an auth token is present, valid, and
/// contains the admin role.
pub fn authenticated_admin() -> impl Filter<Extract = (Jwt,), Error = Rejection> + Copy {
    auth_validation(Role::Admin)
}

/// Filter which checks whether an auth token is present, valid, and
/// contains the specified role.
fn auth_validation(role: Role) -> impl Filter<Extract = (Jwt,), Error = Rejection> + Copy {
    warp::header::headers_cloned()
        .map(move |header_map| (role, header_map))
        .and_then(validate)
}

async fn validate((role, header_map): (Role, HeaderMap)) -> Result<Jwt, Rejection> {
    log::info!("validating auth");

    let auth_header = header_map
        .get(AUTHORIZATION)
        .ok_or(Error::MissingAuthHeader)?
        .to_str()
        .map_err(|_| Error::InvalidAuthHeader)?;

    if !auth_header.starts_with(BEARER) {
        return Err(Error::InvalidAuthHeader.into());
    }

    let token = auth_header.trim_start_matches(BEARER).trim_end();
    let jwt = Jwt::from_auth_token(token, role)?;

    log::info!("authenticated: {}", jwt.id_user());

    Ok(jwt)
}
