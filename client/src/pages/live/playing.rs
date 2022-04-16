use crate::{
    components::{Board, Chat, Rack, Scoreboard},
    pages::live::app_state::{AppMsg, AppState},
};
use api::routes::live::ClientMsg;
use futures::{channel::mpsc, SinkExt};
use sycamore::{
    futures::{spawn_local, spawn_local_scoped},
    prelude::*,
};

/// Props for `Playing`.
#[derive(Prop)]
pub struct Props<'a> {
    /// A read-only signal for the current state.
    pub state: &'a ReadSignal<AppState>,
    /// Writing to this queue sends a message to the dispatch function.
    pub dispatch_write: mpsc::UnboundedSender<AppMsg>,
    /// Writing to this queue sends a message to the server.
    pub ws_write: mpsc::UnboundedSender<ClientMsg>,
}

/// Component for playing a live game.
#[component]
pub fn Playing<'a, G: Html>(cx: Scope<'a>, props: Props<'a>) -> View<G> {
    let state = create_memo(cx, || match props.state.get().as_ref() {
        AppState::Playing(playing_state) => playing_state.clone(),
        AppState::Connected => unreachable!(),
    });

    let rack = state.get().local_rack.clone();
    let tiles = state.get().local_tiles.clone();
    let messages = state.get().messages.clone();
    let scores = create_memo(cx, || state.get().scores.clone());

    // called when a board square is clicked.
    let on_click_square = {
        let dispatch_write = props.dispatch_write.clone();
        move |pos| {
            let mut dispatch_write = dispatch_write.clone();
            spawn_local(async move {
                dispatch_write
                    .send(AppMsg::SquareClicked(pos))
                    .await
                    .unwrap();
            });
        }
    };

    // called when a rack tile is clicked.
    let on_click_rack_tile = {
        let dispatch_write = props.dispatch_write.clone();
        move |tile| {
            let mut dispatch_write = dispatch_write.clone();
            spawn_local(async move {
                dispatch_write
                    .send(AppMsg::RackTileClicked(tile))
                    .await
                    .unwrap();
            });
        }
    };

    // called when a message is sent.
    let on_msg = move |msg| {
        let mut ws_write = props.ws_write.clone();
        spawn_local_scoped(cx, async move {
            ws_write.send(ClientMsg::Chat(msg)).await.unwrap();
        });
    };

    view! { cx,
        div(class="live") {
            Board {
                on_click: on_click_square,
                cells: tiles,
            }

            Rack {
                on_click: on_click_rack_tile,
                tiles: rack,
            }

            // Controls {

            // }

            Scoreboard {
                scores: scores,
            }

            Chat {
                on_msg: on_msg,
                messages: messages,
            }
        }
    }
}
