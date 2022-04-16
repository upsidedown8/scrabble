use crate::{
    components::{Board, Chat, Rack, Scoreboard},
    pages::live::app_state::AppState,
};
use api::routes::live::{ClientMsg, Player};
use futures::{channel::mpsc, SinkExt};
use sycamore::{futures::spawn_local_scoped, prelude::*};

/// Props for `Playing`.
#[derive(Prop)]
pub struct Props<'a> {
    /// A read-only signal for the current state.
    pub state: &'a ReadSignal<AppState>,
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
    let ws_write = create_ref(cx, props.ws_write);

    // -- LOCAL STATE --
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

    // called when a message is sent.
    let on_msg = move |msg| {
        spawn_local_scoped(cx, async move {
            let mut ws_write = ws_write.clone();
            ws_write.send(ClientMsg::Chat(msg)).await.unwrap();
        });
    };

    // whether it is the connected player's turn.
    let is_my_turn = create_memo(cx, || {
        let state = state.get();
        let state = state.as_ref().as_ref();

        matches!(state.next, Some(Player { id_player, .. }) if id_player == state.id_player)
    });

    view! { cx,
        div(class="live") {
            Board {
                on_click: |_| (),
                cells: tiles,
            }

            Rack {
                on_click: |_| (),
                tiles: rack,
            }

            (match *is_my_turn.get() {
                false => view! { cx, },
                true => view! { cx,
                    Controls {
                        ws_write: ws_write.clone(),
                    }
                },
            })

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

/// The tab of the controls menu.
#[derive(PartialEq)]
enum ControlTab {
    Redraw,
    Place,
    Pass,
}

/// Props for `Controls`
#[derive(Prop)]
struct ControlsProps {
    /// Write half of the mpsc queue.
    pub ws_write: mpsc::UnboundedSender<ClientMsg>,
}

#[component]
fn Controls<G: Html>(cx: Scope, props: ControlsProps) -> View<G> {
    // -- TABS --
    let active_tab = create_signal(cx, ControlTab::Place);
    let tab_class = |tab| {
        create_memo(cx, move || match *active_tab.get() == tab {
            true => "is-active",
            false => "",
        })
    };
    let redraw_class = tab_class(ControlTab::Redraw);
    let place_class = tab_class(ControlTab::Place);
    let pass_class = tab_class(ControlTab::Pass);

    view! { cx,
        div(class="controls") {
            div(class="tabs") {
                ul {
                    li(class=(redraw_class.get()), on:click=|_| active_tab.set(ControlTab::Redraw)) { a { "Redraw" } }
                    li(class=(place_class.get()), on:click=|_| active_tab.set(ControlTab::Place)) { a { "Place" } }
                    li(class=(pass_class.get()), on:click=|_| active_tab.set(ControlTab::Pass)) { a { "Pass" } }
                }
            }

            (match *active_tab.get() {
                ControlTab::Redraw => view! { cx, "Redraw" },
                ControlTab::Place => view! { cx, "Place" },
                ControlTab::Pass => view! { cx, "Pass" },
            })
        }
    }
}
