use crate::contexts::get_token;
use reqwest::{Client, Method};
use serde::{de::DeserializeOwned, Serialize};

pub mod users;

static API_URL: &str = "https://localhost:8000/api";

pub async fn make_request<T, U>(url: &str, data: &T, method: Method) -> anyhow::Result<U>
where
    T: Serialize,
    U: DeserializeOwned,
{
    let request_url = format!("{API_URL}{url}");

    let mut client = Client::new().request(method, &request_url).json(data);
    if let Some(token) = get_token() {
        client = client.bearer_auth(token);
    }

    let response = client.send().await?;
    let res = response.json::<U>().await?;

    Ok(res)
}
