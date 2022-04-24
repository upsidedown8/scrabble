//! Component that handles game creation.

use crate::components::{Counter, FixedCounter, Toast};
use api::routes::live::{AiDifficulty, ClientMsg};
use sycamore::prelude::*;
use tokio::sync::mpsc;

/// The active tab.
#[derive(PartialEq)]
enum Tab {
    /// Tab for creating a game.
    Create,
    /// Tab for joining a game.
    Join,
}

/// Props for `CreateOrJoin`.
#[derive(Prop, Clone)]
pub struct Props {
    /// The write half of an unbounded mpsc queue. Messages sent
    /// here will be forwarded to the server.
    pub ws_write: mpsc::UnboundedSender<ClientMsg>,
    /// The pop-up message to display.
    pub msg: RcSignal<Option<String>>,
    /// Whether to show the help modal.
    pub show_modal: RcSignal<bool>,
}

/// Allows a user to create or join a game.
#[component]
pub fn CreateOrJoin<G: Html>(cx: Scope, props: Props) -> View<G> {
    // store the current tab.
    let tab = create_signal(cx, Tab::Create);
    let tab_class = |t| {
        create_memo(cx, move || match *tab.get() == t {
            true => "is-active",
            false => "",
        })
    };

    // signals for the class of each tab (toggles the "is-active"
    // class depending on whether the tab is selected).
    let join_class = tab_class(Tab::Join);
    let create_class = tab_class(Tab::Create);

    // the signal for the toast pop-up.
    let msg = props.msg.clone();

    // signal for the modal help.
    let show_modal = create_ref(cx, props.show_modal.clone());
    let modal_class = create_memo(cx, || match *show_modal.get() {
        true => "modal is-active",
        false => "modal",
    });
    let on_close_modal = |_| {
        show_modal.set(false);
    };

    view! { cx,
        Toast {
            msg: msg,
        }

        div(class=(modal_class.get())) {
            div(class="modal-background")
            div(class="modal-card") {
                header(class="modal-card-head") {
                    p(class="modal-card-title") {
                        "Live games"
                    }
                    button(class="delete", on:click=on_close_modal)
                }
                section(class="modal-card-body") {
                    div(class="content") {
                        h1 { "Joining a game" }
                        p {
                            "You need the " code { "id_game" } " which is displayed in the first message sent
                            when a user joins a game. Note that you cannot join a game that is set to friends
                            only unless you are a friend of the creator of the game."
                        }

                        h1 { "Creating a a game" }
                        p {
                            "When creating a game there are four options to configure:"
                            ul {
                                li {
                                    code { "Player count" } "The total number of players in the game (this includes
                                    AI players)."
                                }
                                li {
                                    code { "Ai count" } "The number of computer players in the game."
                                }
                                li {
                                    code { "Ai difficulty" } "Whether to play against easy, medium or hard Ai. This
                                    only applies if the Ai count is non-zero."
                                }
                                li {
                                    code { "Friends only" } "If this field is checked, only users that you have
                                    added as friends can join the game."
                                }
                            }
                        }
                        p {
                            "After the game is created, send the " code { "id_game" } "at the bottom of the page
                            to a friend so that they can join."
                        }
                    }
                }
                footer(class="modal-card-foot") {
                    button(class="button is-primary", on:click=on_close_modal) {
                        "Close"
                    }
                }
            }
        }

        div(class="page create-game") {
            section(class="is-fullheight columns is-centered is-vcentered is-flex") {
                div(class="box has-text-centered") {
                    div(class="tabs is-centered") {
                        ul {
                            li(class=create_class, on:click=|_| tab.set(Tab::Create)) { a { "Create" } }
                            li(class=join_class, on:click=|_| tab.set(Tab::Join)) { a { "Join" } }
                        }
                    }

                    ({
                        let props = props.clone();

                        match *tab.get() {
                            Tab::Join => view! { cx, JoinTab(props.clone()) },
                            Tab::Create => view! { cx, CreateTab(props.clone()) },
                        }
                    })
                }
            }
        }
    }
}

/// The tab for creating a game.
#[component]
fn CreateTab<G: Html>(cx: Scope, props: Props) -> View<G> {
    // input signals
    let player_count = create_signal(cx, 2);
    let ai_count = create_signal(cx, 0);
    let ai_difficulty = create_signal(cx, AiDifficulty::Medium);
    let friends_only = create_signal(cx, true);

    // the maximum number of ai players.
    let ai_count_max = create_memo(cx, || *player_count.get() - 1);

    // the class signal for a particular ai difficulty button.
    let ai_btn_class = move |difficulty| {
        create_memo(cx, move || match *ai_difficulty.get() == difficulty {
            true => "button is-small is-primary",
            false => "button is-small",
        })
    };
    let ai_btn_on_click = |difficulty| move |_| ai_difficulty.set(difficulty);

    // called when the create button is clicked.
    let on_create = move |_| {
        let player_count = *player_count.get();
        let ai_count = *ai_count.get();
        let ai_difficulty = *ai_difficulty.get();
        let friends_only = *friends_only.get();

        props
            .ws_write
            .send(ClientMsg::Create {
                ai_count,
                ai_difficulty,
                // `player_count` stores the total capacity, but the API expects
                // the number of human players, so subtract `ai_count`.
                player_count: player_count - ai_count,
                friends_only,
            })
            .unwrap();
    };

    view! { cx,
        div(class="field") {
            label(class="label") { "Player count (2-4)" }
            div(class="control") {
                FixedCounter {
                    count: player_count,
                    min: 2,
                    max: 4,
                }
            }
        }

        div(class="field") {
            label(class="label") { "Ai count (0-" (ai_count_max.get()) ")" }
            div(class="control") {
                Counter {
                    min: 0,
                    max: ai_count_max,
                    count: ai_count,
                }
            }
        }

        label(class="label") { "Ai difficulty" }
        div(class="buttons is-centered") {
            a(
                class=ai_btn_class(AiDifficulty::Easy),
                on:click=ai_btn_on_click(AiDifficulty::Easy),
            ) {
                "Easy"
            }
            a(
                class=ai_btn_class(AiDifficulty::Medium),
                on:click=ai_btn_on_click(AiDifficulty::Medium),
            ) {
                "Medium"
            }
            a(
                class=ai_btn_class(AiDifficulty::Hard),
                on:click=ai_btn_on_click(AiDifficulty::Hard),
            ) {
                "Hard"
            }
        }

        div(class="field") {
            label(class="label") {
                input(type="checkbox", bind:checked=friends_only)
                " Friends only?"
            }
        }

        hr

        button(class="button is-primary", on:click=on_create) {
            "Create"
        }
    }
}

/// The tab for joining a game.
#[component]
fn JoinTab<G: Html>(cx: Scope, props: Props) -> View<G> {
    let id_game = create_signal(cx, String::new());

    // called when the user clicks the join button.
    let on_join = move |_| {
        log::info!("join clicked");

        if let Ok(id_game) = id_game.get().parse::<i32>() {
            props.ws_write.send(ClientMsg::Join(id_game)).unwrap();
        } else {
            log::error!("failed to parse game id");
        }
    };

    view! { cx,
        div(class="field") {
            label(class="label") { "Game Id" }
            div(class="control") {
                input(class="input", type="number", min="0", bind:value=id_game)
            }
        }

        hr

        button(class="button is-primary", on:click=on_join) {
            "Join"
        }
    }
}
