use crate::{
    components::{Board, Chat, Rack, Scoreboard},
    pages::live::app_state::AppState,
};
use api::routes::live::{ClientMsg, Player};
use scrabble::game::play::Play;
use sycamore::prelude::*;
use tokio::sync::mpsc;

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
    let ws_write = create_ref(cx, props.ws_write);
    let state = match props.state.get().as_ref() {
        AppState::Playing(playing_state) => playing_state.clone(),
        AppState::Connected(..) => unreachable!(),
    };

    // -- SHARED STATE --
    let tiles = create_ref(cx, state.tiles.clone());
    let rack = create_ref(cx, state.rack.clone());
    let messages = create_ref(cx, state.messages.clone());
    let scores = create_ref(cx, state.scores.clone());

    // whether the game has started.
    let is_playing = create_ref(cx, state.is_playing.clone());
    let next = state.next.clone();
    // whether it is the connected player's turn.
    let is_my_turn = create_memo(cx, move || {
        let next = next.get();
        let is_playing = *is_playing.get();

        is_playing
            && matches!(next.as_ref(), Some(Player { id_player, .. }) if *id_player == state.id_player)
    });

    // -- LOCAL STATE --
    let local_tiles = create_signal(cx, vec![]);
    create_effect(cx, || local_tiles.set((*tiles.get()).clone()));
    let local_rack = create_signal(cx, vec![]);
    create_effect(cx, || local_rack.set((*rack.get()).clone()));
    // let selected_tile = create_signal(cx, None);
    // create_effect(cx, || {
    //     // whenever the rack changes, reset the selected tile to None.
    //     rack.track();
    //     selected_tile.set(None);
    // });

    // -- CALLBACKS --
    // called when a chat message is sent.
    let on_chat_msg = move |msg| {
        ws_write.send(ClientMsg::Chat(msg)).unwrap();
    };
    // called when a board square is clicked.
    let on_square_clicked = |pos| {
        log::info!("{pos} clicked");
    };
    // called when a rack tile is clicked.
    let on_rack_tile_clicked = |idx, tile| {
        log::info!("rack tile {tile} at {idx} clicked");
    };

    view! { cx,
        div(class="live") {
            Board {
                on_click: on_square_clicked,
                cells: tiles,
            }

            Rack {
                on_click: on_rack_tile_clicked,
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
                on_msg: on_chat_msg,
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
    let ws_write = create_ref(cx, props.ws_write);

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
    // called when the user clicks the pass button.
    let on_pass = move |_| {
        ws_write.send(ClientMsg::Play(Play::Pass)).unwrap();
    };
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
