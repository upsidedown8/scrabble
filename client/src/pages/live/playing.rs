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
    let local_rack = create_ref(cx, state.rack.clone());
    let placed_tiles = create_ref(cx, state.placed_tiles.clone());
    let redraw_tiles = create_ref(cx, state.redraw_tiles.clone());
    let messages = create_ref(cx, state.messages.clone());
    let scores = create_ref(cx, state.scores.clone());
    let letter_bag_remaining = create_ref(cx, state.letter_bag_len.clone());
    let show_rules_modal = create_ref(cx, state.show_rules_modal.clone());

    // whether the game has started.
    let is_started = create_ref(cx, state.is_started.clone());
    let is_over = create_ref(cx, state.is_over.clone());
    let next = state.next.clone();

    // whether it is the connected player's turn.
    let is_my_turn = create_memo(cx, move || {
        let next = next.get();
        let is_started = *is_started.get();

        is_started
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
    let selected_tile = create_ref(cx, create_rc_signal(None));

    let blank_tile = create_signal(cx, None);
    let show_modal = create_memo(cx, || blank_tile.get().is_some());
    let modal_class = create_memo(cx, || match *show_modal.get() {
        true => "modal is-active",
        false => "modal",
    });
    let modal_letter = create_signal(cx, String::new());

    // -- STATE FOR PLAYS --
    let redraw_selected = create_signal(cx, None);

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
        } else {
            selected_tile.set(None);
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

    let rules_modal_class = create_memo(cx, || match *show_rules_modal.get() {
        true => "modal is-active",
        false => "modal",
    });
    let on_close_rules = |_| {
        show_rules_modal.set(false);
    };

    view! { cx,
        // This modal displays rules and information about the game.
        div(class=(rules_modal_class.get())) {
            div(class="modal-background")
            div(class="modal-card") {
                header(class="modal-card-head") {
                    p(class="modal-card-title") {
                        "How to play"
                    }
                    button(class="delete", on:click=on_close_rules)
                }
                section(class="modal-card-body") {
                    div(class="content") {
                        h1 { "Rules" }
                        p {
                            "View the official rules "
                            a(href="https://scrabble.hasbro.com/en-us/rules", target="_blank") { "here" }
                            "."
                        }

                        h1 { "Joining a game" }
                        p {
                            "When you join or create a game you will see this page.
                            A message at the bottom of the page indicates
                            the " code { "id_game" } " which can be used on the Join page
                            by other players to join the current game. Note that if the game
                            was created with the " code { "Friends only" } " option checked,
                            only users that have been added as friends can join the game."
                        }

                        h1 { "Messages" }
                        p {
                            "At the bottom of the page is a message box. Typing a message
                            and pressing enter will send it to other connected players.
                            Messages from the server will indicate when:"

                            ul {
                                li { "A play is made" }
                                li { "A player joins or leaves" }
                                li { "The game starts or ends" }
                                li { "You make an illegal play" }
                            }
                        }

                        h1 { "Playing" }
                        h2 { "Premium squares" }
                        p {
                            "Hovering over (or clicking on mobile devices) a square will show
                            the premium for that square (if any). Premium squares are coloured
                            on the board."

                            table {
                                thead {
                                    th { "Abbreviation" }
                                    th { "Meaning" }
                                }
                                tbody {
                                    tr {
                                        td { "2L" }
                                        td { "Double letter" }
                                    }
                                    tr {
                                        td { "3L" }
                                        td { "Triple letter" }
                                    }
                                    tr {
                                        td { "2W" }
                                        td { "Double word" }
                                    }
                                    tr {
                                        td { "3W" }
                                        td { "Triple word" }
                                    }
                                }
                            }
                        }

                        h2 { "Remaining tile count" }
                        p {
                            "A counter under your rack tiles indicates the number of tiles that remain
                            in the letter bag."
                        }

                        h2 { "Blank tiles" }
                        p {
                            "When you have a blank tile in your rack, you can place it on the board
                            as you would any other tile. After placing the tile, a dialogue box
                            will ask for the letter that you wish the tile to represent."
                        }
                        p {
                            "When you see a blank tile on the board, you can hover (or tap on mobile)
                            the tile to reveal its letter."
                        }

                        h2 { "Sorting tiles on your rack" }
                        p {
                            "To reorder tiles, simply click on one tile in rack to select it, then
                            click another to swap the pair. (tiles cannot be reordered whilst in
                            the redraw tab)."
                        }

                        h2 { "Making a play" }
                        p {
                            "When it is your turn, a view with three tabs will
                            appear that allows you to make a play."
                        }

                        h3 { "Place tab" }
                        p {
                            "Click on a tile in your rack, then an empty board square to place a tile.
                            If you wish to return a tile to your rack, click on that tile on the board.
                            To quickly return all placed tiles to your rack, there is a"
                            code { "Recall these tiles" } " button. Once you are happy with the positions of your
                            tiles, clicking " code { "Place these tiles" } " will make the play."
                        }

                        h3 { "Redraw tab" }
                        p {
                            "Clicking the " code { "Select all tiles" } " button will move
                            all tiles from your rack into the redraw area. Clicking tiles in
                            the redraw area will return them to your rack. If you wish to select
                            specific tiles to redraw, click on tiles in your rack and they will move
                            to the redraw area."
                        }
                        p {
                            "Once you have selected the tiles you wish to redraw, clicking the "
                            code { "Redraw these tiles" } " button will make the play."
                        }

                        h3 { "Pass tab" }
                        p {
                            "Clicking the " code { "Pass" } " button will pass your turn. Note that
                            after two consecutive passes from any player, the game will end."
                        }

                        h2 { "Illegal plays" }
                        p {
                            "If you make an illegal play, your tiles will return to your rack. A message
                            at the bottom of the page will indicate the reason that your play was rejected. "
                        }

                        h2 { "End of your turn" }
                        p {
                            "After making your play, the Redraw, Place and Pass tabs will disappear
                            until it is your turn again."
                        }

                        h2 { "End of the game" }
                        p {
                            "At the end of the game, the final scores will update and a message
                            indicating the reason for the end of the game will be sent at the bottom
                            of the page. Players may remain in the game after its end to send messages,
                            but no further plays can be made. Once the game ends, your statistics will update."
                        }

                        h2 { "Rejoining a game" }
                        p {
                            "If you disconnect from an ongoing game and no other users remain
                            (AI players do not count) then the game will close permanently within the next
                            ten seconds. If, however, some users remain in the game, you will be replaced by
                            an AI player until you rejoin the game."
                        }
                    }
                }
                footer(class="modal-card-foot") {
                    button(class="button is-primary", on:click=on_close_rules) {
                        "Close"
                    }
                }
            }
        }

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

                p(class="pb-4 has-text-centered has-text-white") {
                    (match *is_started.get() {
                        false => view! { cx, "Waiting for players" },
                        true => view! { cx,
                            (match *is_over.get() {
                                true => view! { cx, "Game over" },
                                false => view! { cx, "There are " (letter_bag_remaining.get()) " tiles remaining" }
                            })
                        }
                    })
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

            div(class="mb-5 p-0 has-background-white has-text-centered") {
                a(class="button is-primary", on:click=|_| show_rules_modal.set(true)) {
                    "Help"
                }
            }
        }
    }
}
