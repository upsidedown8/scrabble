use api::auth::Auth;
use serde::{de::DeserializeOwned, Serialize};
use reqwasm::http::{Request, Method};
use wasm_bindgen::JsValue;
use sycamore::prelude::ScopeRef;
use crate::{contexts::auth::{use_auth_ctx, AuthCtx}, error::Error};

pub mod users;

const API_URL: &str = "https://localhost:8000/api";

pub async fn make_request<T, U>(ctx: ScopeRef<'_>, url: &str, data: &T, method: Method) -> Result<U, Error>
where
    T: Serialize,
    U: DeserializeOwned,
{
    let request_url = format!("{API_URL}{url}");

    let body_js = JsValue::from_serde(data)?;
    let mut req = Request::new(&request_url)
        .method(method)
        .body(body_js);

    if let Some(AuthCtx { auth, .. }) = use_auth_ctx(ctx).get().as_ref() {
        let Auth(token) = auth;
        let bearer = format!("Bearer {token}");

        req = req.header("Authorization", &bearer);
    }

    let response = req.send().await?;
    let res = response.json::<U>().await?;

    Ok(res)
}
