use crate::{
    components::{Board, Chat, Rack, Scoreboard},
    pages::live::app_state::AppState,
};
use api::routes::live::{ClientMsg, Player};
use scrabble::{game::tile, util::pos::Pos};
use sycamore::prelude::*;
use tokio::sync::mpsc;

/// a rack tile or board square that has been selected.
#[derive(Clone, Copy)]
pub enum Selected {
    /// A tile on the rack has been selected.
    RackTile(tile::Tile),
    /// A board position has been selected.
    Square(Pos),
}

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
        ws_write.send(ClientMsg::Chat(msg)).unwrap();
    };

    // whether it is the connected player's turn.
    let is_my_turn = create_memo(cx, || {
        let state = state.get();
        let state = state.as_ref().as_ref();

        matches!(state.next, Some(Player { id_player, .. }) if id_player == state.id_player)
    });

    // the rack or board tile that is selected.
    let selected: &Signal<Option<Selected>> = create_signal(cx, None);

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

    // -- CALLBACKS --
    let on_pass = |_| {};
    let on_redraw = |_| {};

    view! { cx,
        div(class="controls") {
            div(class="tabs is-centered") {
                ul {
                    li(class=(redraw_class.get()), on:click=|_| active_tab.set(ControlTab::Redraw)) { a { "Redraw" } }
                    li(class=(place_class.get()), on:click=|_| active_tab.set(ControlTab::Place)) { a { "Place" } }
                    li(class=(pass_class.get()), on:click=|_| active_tab.set(ControlTab::Pass)) { a { "Pass" } }
                }
            }

            section {
                (match *active_tab.get() {
                    ControlTab::Redraw => view! { cx,
                        p { "Select tiles from your rack to redraw" }

                        button(class="button is-dark", on:click=on_redraw) {
                            "Redraw these tiles"
                        }
                    },
                    ControlTab::Place => view! { cx, "Place" },
                    ControlTab::Pass => view! { cx,
                        button(class="button is-dark", on:click=on_pass) {
                            "Pass your turn"
                        }
                    },
                })
            }
        }
    }
}
