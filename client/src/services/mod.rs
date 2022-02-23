//! Methods for interacting with the API asynchronously.

use crate::{
    contexts::{AuthCtx, AuthSignal},
    error::Error,
};
use api::auth::Auth;
use reqwasm::http::{Method, Request};
use serde::{de::DeserializeOwned, Serialize};

pub mod users;

const API_URL: &str = "https://localhost:8000/api";

/// Make a request to the path {API_URL}/{url}, with the provided
/// method and data.
pub async fn make_request<T, U>(
    auth_ctx: &AuthSignal,
    url: &str,
    data: &T,
    method: Method,
) -> Result<U, Error>
where
    T: Serialize,
    U: DeserializeOwned,
{
    let request_url = format!("{API_URL}{url}");

    let body = serde_json::to_string(data)?;

    log::info!("method: {method}, url: {request_url}, body: {body}");

    let mut req = Request::new(&request_url)
        .method(method)
        .header("Content-Type", "Application/JSON")
        .body(body);

    log::info!("{req:#?}");

    if let Some(AuthCtx { auth, .. }) = auth_ctx.get().as_ref() {
        let Auth(token) = auth;
        let bearer = format!("Bearer {token}");

        req = req.header("Authorization", &bearer);

        log::info!("added auth header");
    }

    let response = req.send().await?;

    log::info!("{response:?}");

    // match on the response http status and return either
    // an error message or the deserialized content.
    match response.status() {
        // (200 OK) or (201 CREATED)
        200 | 201 => Ok(response.json().await?),
        // 400 BAD_REQUEST
        400 => Err(Error::BadRequest),
        // 401 UNAUTHORIZED
        401 => Err(Error::Unauthorized),
        // 403 FORBIDDEN
        403 => Err(Error::Forbidden),
        404 => Err(Error::NotFound),
        500 => Err(Error::InternalServerError),
        status => Err(Error::HttpStatus(status)),
    }
}
