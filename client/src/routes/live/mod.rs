//! Implementation of the [`LivePage`].

use crate::{components::Board, contexts::ScopeExt};
use api::routes::live::{ClientMsg, LiveError, ServerMsg};
use futures::{channel::mpsc, SinkExt, StreamExt};
use reqwasm::websocket::{futures::WebSocket, Message};
use scrabble::util::pos::Pos;
use sycamore::{futures::ScopeSpawnLocal, prelude::*};

/// Properties for the live page.
#[derive(Prop)]
pub struct Props {
    /// Id of the live game to join.
    pub id_game: Option<i32>,
}

/// Page for playing live games.
#[component]
pub fn LivePage<G: Html>(ctx: ScopeRef, props: Props) -> View<G> {
    let auth_ctx = ctx.use_auth_context();

    let msg = ctx.create_signal(String::new());
    let recv = ctx.create_signal(String::new());
    let ws = WebSocket::open("wss://localhost:8000/api/games/join").unwrap();

    let (mut write, mut read) = ws.split();
    let (tx, mut rx) = mpsc::unbounded();
    let tx = ctx.create_signal(tx);

    // let auth_msg = GameMessage::Authenticate();
    // write.send(Message::Bytes(bincode::serialize(&auth_msg).unwrap()));

    ctx.spawn_local(async move {
        while let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Text(data)) => {
                    log::debug!("recieved: {data:?}");
                    recv.set(data);
                }
                Ok(Message::Bytes(data)) => log::debug!("recieved: {data:?}"),
                Err(e) => log::error!("{e:?}"),
            }
        }
        log::info!("websocket closed");
    });
    ctx.spawn_local(async move {
        while let Some(msg) = rx.next().await {
            log::debug!("send: {msg:?}");
            write.send(Message::Text(msg)).await.unwrap();
        }
    });

    let onsend = |_| {
        ctx.spawn_local(async {
            let mut tx = (*tx.get()).clone();
            let msg = String::clone(&msg.get());

            tx.send(msg).await.unwrap();
        });
    };

    let cells = ctx.create_signal((0..225).map(|n| (Pos::from(n), None)).collect::<Vec<_>>());

    view! { ctx,
        div(class="play-route is-centered is-vcentered is-flex columns") {
            div(class="box") {
                div(class="field") {
                    input(class="input",type="text", bind:value=msg)
                }

                div(class="field") {
                    textarea(class="input", disabled=true) {
                        (recv.get())
                    }
                }

                Board {
                    cells: cells,
                }

                button(class="button is-primary is-light",on:click=onsend) {
                    "Send"
                }
            }
        }
    }
}
