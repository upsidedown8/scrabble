//! Methods for interacting with the API asynchronously.

use crate::{
    context::{set_token, AuthCtx, AuthSignal},
    error::{Error, Result},
};
use api::auth::{AuthWrapper, Token};
use reqwasm::http::{Method, Request};
use serde::{de::DeserializeOwned, Serialize};

pub mod friends;
pub mod games;
pub mod leaderboard;
pub mod live;
pub mod users;

/// The domain name and path to the API, excluding the protocol.
pub const API_HOST: &str = "localhost/api";

/// Make a request to the path {API_URL}/{url}, with the provided
/// method and data. Returns the optional auth from the server and
/// the requested value.
pub async fn request<T, U>(
    url: &str,
    method: Method,
    data: Option<&T>,
    auth: Option<&AuthSignal>,
) -> Result<(Option<Token>, U)>
where
    T: Serialize,
    U: DeserializeOwned,
{
    let mut req = Request::new(&format!("https://{API_HOST}{url}")).method(method);

    // Add the JSON body.
    if let Some(data) = data {
        let body = serde_json::to_string(data)?;
        req = req.header("Content-Type", "Application/JSON").body(body);
        log::debug!("added json body");
    }

    // Add the auth header
    if let Some(auth_signal) = auth {
        if let Some(AuthCtx { token, .. }) = auth_signal.get().as_ref() {
            let Token(token) = token;
            req = req.header("Authorization", &format!("Bearer {token}"));
            log::debug!("added auth header");
        }
    }

    log::info!("sending request: {req:#?}");

    let response = req.send().await?;

    log::info!("response received: {response:?}");

    // match on the response http status and return either
    // an error message or the deserialized content.
    match response.status() {
        // (200 OK) or (201 CREATED)
        200 | 201 => Ok({
            // attempt to deserialize as `AuthWrapper<U>`.
            if let Ok(AuthWrapper { token, response }) = response.json().await {
                (token, response)
            }
            // attempt to deserialize as `U`.
            else {
                log::error!("failed to parse successful response");
                (None, response.json().await?)
            }
        }),
        status => {
            if let Ok(error_response) = response.json().await {
                Err(Error::Api(error_response))
            } else {
                log::error!("failed to parse error response");
                Err(Error::HttpStatus(status))
            }
        }
    }
}

/// Makes a request and updates the auth field if a token is
/// received.
pub async fn req_std<T, U>(
    url: &str,
    method: Method,
    data: Option<&T>,
    auth_signal: Option<&AuthSignal>,
) -> Result<U>
where
    T: Serialize,
    U: DeserializeOwned,
{
    let (token, response) = request(url, method, data, auth_signal).await?;

    // If a new auth token is received, update the stored token.
    if let Some(token) = token {
        if let Some(auth_signal) = auth_signal {
            set_token(auth_signal, token);
        }
    }

    Ok(response)
}

/// Makes a standard request with an empty body.
pub async fn req_no_body<U>(
    url: &str,
    method: Method,
    auth_signal: Option<&AuthSignal>,
) -> Result<U>
where
    U: DeserializeOwned,
{
    req_std(url, method, Option::<&()>::None, auth_signal).await
}
