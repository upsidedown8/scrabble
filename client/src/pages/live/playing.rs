use crate::{
    components::{Board, Chat, Scoreboard, Tiles},
    pages::live::app_state::AppState,
};
use api::routes::live::{ClientMsg, Player};
use scrabble::game::{
    play::Play,
    tile::{Letter, Tile},
};
use sycamore::{prelude::*, rt::JsCast};
use tokio::sync::mpsc;
use web_sys::{Event, KeyboardEvent};

/// The tab of the controls menu.
#[derive(PartialEq)]
enum ControlTab {
    Redraw,
    Place,
    Pass,
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

    // -- LOCAL STATE --
    let local_tiles = create_signal(cx, vec![]);
    create_effect(cx, || local_tiles.set((*tiles.get()).clone()));
    let local_rack = create_signal(cx, vec![]);
    create_effect(cx, || local_rack.set((*rack.get()).clone()));
    let selected_tile = create_ref(cx, create_rc_signal(None));

    let blank_tile = create_signal(cx, None);
    let show_modal = create_memo(cx, || blank_tile.get().is_some());
    let modal_class = create_memo(cx, || match *show_modal.get() {
        true => "modal is-active",
        false => "modal",
    });
    let modal_letter = create_signal(cx, String::new());

    // -- STATE FOR PLAYS --
    let redraw_tiles = create_signal(cx, vec![]);
    let redraw_selected = create_signal(cx, None);
    let placed_tiles = create_signal(cx, vec![]);

    // -- CALLBACKS --
    // called when a chat message is sent.
    let on_chat_msg = move |msg| {
        ws_write.send(ClientMsg::Chat(msg)).unwrap();
    };
    // called when a board square is clicked.
    let on_square_clicked = |pos| {
        log::info!("{pos} clicked");

        // only handle square clicks when it is the player's turn.
        if *is_my_turn.get() {
            // only handle square clicks if the place tab is selected.
            if *active_tab.get() == ControlTab::Place {
                if let Some((idx, tile)) = *selected_tile.get() {
                    selected_tile.set(None);

                    // if the board position is empty, place the tile.
                    let mut local_tiles = local_tiles.modify();
                    if local_tiles[usize::from(pos)].is_none() {
                        match tile {
                            // if the tile is a letter, place it on the board.
                            Tile::Letter(_) => {
                                local_tiles[usize::from(pos)] = Some(tile);
                                local_rack.modify().remove(idx);
                                placed_tiles.modify().push((pos, tile));
                            }
                            // if the tile is blank, show a modal to determine which letter
                            // it should be.
                            Tile::Blank(_) => blank_tile.set(Some((idx, pos))),
                        }
                    }
                } else {
                    // if no tile is selected, and the position clicked was
                    // newly placed, return that tile to the rack.
                    let placed = placed_tiles.get();
                    let newly_placed = placed.iter().find(|(p, _)| *p == pos);

                    if let Some((_, tile)) = newly_placed {
                        let mut local_tiles = local_tiles.modify();
                        local_tiles[usize::from(pos)] = None;
                        placed_tiles.modify().retain(|(p, _)| *p != pos);
                        local_rack.modify().push(*tile);
                    }
                }
            }
        }
    };
    // called when a rack tile is clicked.
    let on_rack_tile_clicked = |idx, tile| {
        log::info!("rack tile {tile} at {idx} clicked");

        match *active_tab.get() {
            // When redrawing, sent tiles directly to the redraw list.
            ControlTab::Redraw => {
                selected_tile.set(None);
                local_rack.modify().remove(idx);
                redraw_tiles.modify().push(tile);
            }
            _ => {
                match *selected_tile.get() {
                    // Two tiles on the rack were clicked.
                    Some((i, _)) => {
                        // Deselect the tile.
                        selected_tile.set(None);

                        // Swap the tiles at `i` and `idx` in the rack.
                        local_rack.modify().swap(idx, i);
                    }
                    None => selected_tile.set(Some((idx, tile))),
                }
            }
        }
    };
    // called when the recall button is clicked.
    let on_recall = |_| {
        let mut local_tiles = local_tiles.modify();
        let mut local_rack = local_rack.modify();

        for (pos, tile) in placed_tiles.modify().drain(..) {
            local_tiles[usize::from(pos)] = None;
            local_rack.push(tile);
        }
    };
    // called when a tile to redraw is clicked.
    let on_redraw_tile_clicked = |idx, tile| {
        redraw_tiles.modify().remove(idx);
        local_rack.modify().push(tile);
    };
    // called when a key is pressed in the modal.
    let on_modal_keydown = |evt: Event| {
        let keyboard_event: KeyboardEvent = evt.unchecked_into();
        let key = keyboard_event.key();

        if key.len() == 1 {
            if let Some(ch) = key.chars().next() {
                if let Some(letter) = Letter::new(ch) {
                    // get the tile, rack position (idx) and board position (pos).
                    let tile = Tile::Blank(Some(letter));
                    let (idx, pos) = (*blank_tile.get()).unwrap();
                    blank_tile.set(None);

                    let mut local_tiles = local_tiles.modify();
                    local_rack.modify().remove(idx);
                    local_tiles[usize::from(pos)] = Some(tile);
                    placed_tiles.modify().push((pos, tile));
                }
            }
        }
    };
    // called when the user clicks the pass button.
    let on_pass = move |_| {
        ws_write.send(ClientMsg::Play(Play::Pass)).unwrap();
    };
    // called when the user clicks the redraw button.
    let on_redraw = move |_| {
        let tiles = (*redraw_tiles.get()).clone();
        ws_write.send(ClientMsg::Play(Play::Redraw(tiles))).unwrap();
    };
    // called when the user clicks the place button.
    let on_place = move |_| {
        let tiles = (*placed_tiles.get()).clone();
        ws_write.send(ClientMsg::Play(Play::Place(tiles))).unwrap();
    };
    // redraws all tiles.
    let on_redraw_all = move |_| {
        // remove all tiles from the rack.
        selected_tile.set(None);
        let mut redraw_tiles = redraw_tiles.modify();
        for tile in local_rack.modify().drain(..) {
            redraw_tiles.push(tile);
        }
    };

    view! { cx,
        // This modal will display when a blank tile is placed on the board.
        div(class=(modal_class.get())) {
            div(class="modal-background")
            div(class="modal-content") {
                div(class="box") {
                    div(class="field") {
                        label(class="label") {
                            "Enter a letter for the blank tile"
                        }
                        div(class="control") {
                            input(
                                class="input",
                                type="text",
                                maxlength="1",
                                placeholder="Letter",
                                bind:value=modal_letter,
                                on:keydown=on_modal_keydown,
                            )
                        }
                    }
                }
            }
            button(class="modal-close is-large", on:click=|_| blank_tile.set(None))
        }

        div(class="live") {
            Board {
                on_click: on_square_clicked,
                cells: local_tiles,
            }

            div(class="rack") {
                Tiles {
                    on_click: on_rack_tile_clicked,
                    tiles: local_rack,
                    selected: selected_tile,
                }
            }

            (match *is_my_turn.get() {
                false => view! { cx, },
                true => view! { cx,
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
                                    (match redraw_tiles.get().len() {
                                        0 => view! { cx,
                                            p { "Select tiles from your rack to redraw" }

                                            button(class="button mt-4 is-dark", on:click=on_redraw_all) {
                                                "Select all tiles"
                                            }
                                        },
                                        _ => view! { cx,
                                            div(class="redraw") {
                                                Tiles {
                                                    on_click: on_redraw_tile_clicked,
                                                    tiles: redraw_tiles,
                                                    selected: redraw_selected,
                                                }
                                            }

                                            button(class="button is-dark", on:click=on_redraw) {
                                                "Redraw these tiles"
                                            }
                                        }
                                    })
                                },
                                ControlTab::Place => view! { cx,
                                    (match placed_tiles.get().len() {
                                        0 => view! { cx,
                                            p { "Select tiles from your rack and place them on the board" }
                                        },
                                        _ => view! { cx,
                                            div(class="buttons is-centered") {
                                                button(class="button is-dark", on:click=on_recall) {
                                                    "Recall these tiles"
                                                }

                                                button(class="button is-dark", on:click=on_place) {
                                                    "Place these tiles"
                                                }
                                            }
                                        }
                                    })
                                },
                                ControlTab::Pass => view! { cx,
                                    button(class="button is-dark", on:click=on_pass) {
                                        "Pass your turn"
                                    }
                                },
                            })
                        }
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
