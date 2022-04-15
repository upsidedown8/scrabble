use crate::{error::Result, requests::API_HOST};
use api::{auth::Token, routes::live::ClientMsg};
use futures::SinkExt;
use reqwasm::websocket::{futures::WebSocket, Message};

/// Connects to the live websocket server and authenticates
/// the user.
pub async fn connect_and_authenticate(token: Token) -> Result<WebSocket> {
    let url = format!("wss://{API_HOST}/live");
    let mut ws = WebSocket::open(&url)?;

    // Send a `ClientMsg::Auth` to authenticate the connection.
    ws.send(to_msg(&ClientMsg::Auth(token))).await?;

    Ok(ws)
}

/// Converts a `ClientMsg` to a websocket message.
pub fn to_msg(msg: &ClientMsg) -> Message {
    let bytes = bincode::serialize(msg).unwrap();
    Message::Bytes(bytes)
}
