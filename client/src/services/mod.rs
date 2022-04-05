//! Methods for interacting with the API asynchronously.

use crate::{
    contexts::{AuthCtx, AuthSignal},
    error::{Error, Result},
};
use api::auth::{Auth, AuthWrapper};
use reqwasm::http::{Method, Request};
use serde::{de::DeserializeOwned, Serialize};

pub mod friends;
pub mod games;
pub mod leaderboard;
pub mod users;

const API_URL: &str = "https://thrgd.uk/api";

/// Sends the [`Request`].
async fn send_request<U>(req: Request) -> Result<(Option<Auth>, U)>
where
    U: DeserializeOwned,
{
    let response = req.send().await?;

    log::info!("response received: {response:?}");

    // match on the response http status and return either
    // an error message or the deserialized content.
    match response.status() {
        // (200 OK) or (201 CREATED)
        200 | 201 => Ok({
            // attempt to deserialize as `AuthWrapper<U>`.
            if let Ok(AuthWrapper { auth, response }) = response.json().await {
                (auth, response)
            }
            // attempt to deserialize as `U`.
            else {
                (None, response.json().await?)
            }
        }),
        status @ (400 | 401 | 403 | 404 | 500) => {
            if let Ok(error_response) = response.json().await {
                Err(Error::ApiError(error_response))
            } else {
                Err(Error::HttpStatus(status))
            }
        }
        status => Err(Error::HttpStatus(status)),
    }
}

/// Make a request to the path {API_URL}/{url}, with the provided
/// method and data. Returns the optional auth from the server and
/// the requested value.
pub async fn request<T, U>(
    auth: Option<&Auth>,
    url: &str,
    data: &T,
    method: Method,
) -> Result<(Option<Auth>, U)>
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

    if let Some(Auth(token)) = auth {
        let bearer = format!("Bearer {token}");

        req = req.header("Authorization", &bearer);

        log::info!("added auth header");
    }

    send_request(req).await
}

/// Makes a request with no body.
pub async fn request_no_body<U>(
    auth: Option<&Auth>,
    url: &str,
    method: Method,
) -> Result<(Option<Auth>, U)>
where
    U: DeserializeOwned,
{
    let request_url = format!("{API_URL}{url}");

    log::info!("method: {method}, url: {request_url}, empty body");

    let mut req = Request::new(&request_url).method(method);

    if let Some(Auth(token)) = auth {
        let bearer = format!("Bearer {token}");

        req = req.header("Authorization", &bearer);

        log::info!("added auth header");
    }

    send_request(req).await
}

/// Makes a request and updates the auth field if a token is
/// received.
pub async fn std_request<T, U>(
    auth_ctx: &AuthSignal,
    url: &str,
    data: &T,
    method: Method,
) -> Result<U>
where
    T: Serialize,
    U: DeserializeOwned,
{
    // Extract the token from the auth signal.
    let ctx = auth_ctx.get();
    let token = ctx.as_ref().as_ref().map(|ctx| &ctx.auth);

    // Make the request.
    let (auth, response) = request(token, url, data, method).await?;
    update_auth(auth_ctx, auth);

    // Return the response from the server.
    Ok(response)
}

/// Makes a request with no body and updates the auth field
/// if a token is received.
pub async fn std_request_no_body<U>(auth_ctx: &AuthSignal, url: &str, method: Method) -> Result<U>
where
    U: DeserializeOwned,
{
    // Extract the token from the auth signal.
    let ctx = auth_ctx.get();
    let token = ctx.as_ref().as_ref().map(|ctx| &ctx.auth);

    // Make the request.
    let (auth, response) = request_no_body(token, url, method).await?;
    update_auth(auth_ctx, auth);

    // Return the response from the server.
    Ok(response)
}

/// Updates the global auth state if `auth` is `Some`.
fn update_auth(auth_ctx: &AuthSignal, auth: Option<Auth>) {
    // If a new auth token is received, update the stored token.
    if let Some(auth) = auth {
        let details = auth_ctx
            .get()
            .as_ref()
            .as_ref()
            .map(|auth_ctx| auth_ctx.user_details.clone())
            .unwrap_or_default();
        auth_ctx.set(Some(AuthCtx {
            user_details: details,
            auth,
        }))
    }
}
