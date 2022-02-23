//! Implementation of the [`PlayPage`].

use futures::{channel::mpsc, SinkExt, StreamExt};
use reqwasm::websocket::{futures::WebSocket, Message};
use sycamore::{futures::ScopeSpawnFuture, prelude::*};

/// Page for playing live games.
#[component]
pub fn PlayPage<G: Html>(ctx: ScopeRef) -> View<G> {
    let msg = ctx.create_signal(String::new());
    let recv = ctx.create_signal(String::new());
    let ws = WebSocket::open("wss://localhost:8000/api/games/ws_echo").unwrap();

    let (mut write, mut read) = ws.split();

    let (tx, mut rx) = mpsc::unbounded();
    let tx = ctx.create_signal(tx);

    ctx.spawn_future(async move {
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
    ctx.spawn_future(async move {
        while let Some(msg) = rx.next().await {
            log::debug!("send: {msg:?}");
            write.send(Message::Text(msg)).await.unwrap();
        }
    });

    let onsend = |_| {
        ctx.spawn_future(async {
            let mut tx = (*tx.get()).clone();
            let msg = String::clone(&msg.get());

            tx.send(msg).await.unwrap();
        });
    };

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

                button(class="button is-primary is-light",on:click=onsend) {
                    "Send"
                }
            }
        }
    }
}
