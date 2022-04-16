use crate::{
    components::{Board, Chat, Rack, Scoreboard},
    pages::live::app_state::{AppMsg, AppState},
};
use api::routes::live::ClientMsg;
use futures::{channel::mpsc, SinkExt};
use scrabble::game::play::Play;
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
        AppState::Connected(..) => unreachable!(),
    });

    let rack = create_memo(cx, || {
        let state = state.get();
        let state = state.as_ref().as_ref();
        state.rack.clone()
    });
    let tiles = create_memo(cx, || {
        let state = state.get();
        let state = state.as_ref().as_ref();
        state.tiles.clone()
    });
    let messages = state.get().messages.clone();
    let scores = create_memo(cx, || state.get().scores.clone());

    let send_app_msg = {
        let dispatch_write = props.dispatch_write.clone();
        move |msg| {
            let mut dispatch_write = dispatch_write.clone();
            spawn_local(async move {
                dispatch_write.send(msg).await.unwrap();
            });
        }
    };

    // called when a board square is clicked.
    let on_click_square = {
        let send_msg = send_app_msg.clone();
        move |pos| send_msg(AppMsg::SquareClicked(pos))
    };

    // called when a rack tile is clicked.
    let on_click_rack_tile = {
        let send_msg = send_app_msg;
        move |tile| send_msg(AppMsg::RackTileClicked(tile))
    };

    // called when the pass button is clicked.
    let on_pass = {
        let ws_write = props.ws_write.clone();
        move |_| {
            let mut ws_write = ws_write.clone();
            spawn_local_scoped(cx, async move {
                ws_write.send(ClientMsg::Play(Play::Pass)).await.unwrap();
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

            div(class="controls") {
                div(class="buttons") {
                    button(class="button is-dark") {
                        "Redraw"
                    }
                    button(class="button is-dark") {
                        "Place"
                    }
                    button(class="button is-dark", on:click=on_pass) {
                        "Pass"
                    }
                }
            }

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
