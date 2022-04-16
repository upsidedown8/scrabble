//! Component that handles game creation.

use crate::components::{Counter, FixedCounter};
use api::routes::live::ClientMsg;
use futures::{channel::mpsc, SinkExt};
use sycamore::{futures::spawn_local_scoped, prelude::*};

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

    view! { cx,
        div(class="page create-game") {
            section(class="is-fullheight columns is-centered is-vcentered is-flex") {
                div(class="box has-text-centered") {
                    div(class="tabs") {
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
    let friends_only = create_signal(cx, true);

    // the maximum number of ai players.
    let ai_count_max = create_memo(cx, || *player_count.get() - 1);

    // called when the create button is clicked.
    let on_create = move |_| {
        let mut ws_write = props.ws_write.clone();

        let player_count = *player_count.get();
        let ai_count = *ai_count.get();
        let friends_only = *friends_only.get();

        spawn_local_scoped(cx, async move {
            ws_write
                .send(ClientMsg::Create {
                    ai_count,
                    // `player_count` stores the total capacity, but the API expects
                    // the number of human players, so subtract `ai_count`.
                    player_count: player_count - ai_count,
                    friends_only,
                })
                .await
                .unwrap();
        });
    };

    view! { cx,
        h2(class="title is-5") { "Create game" }

        hr

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
        let mut ws_write = props.ws_write.clone();

        log::info!("join clicked");

        if let Ok(id_game) = id_game.get().parse::<i32>() {
            spawn_local_scoped(cx, async move {
                ws_write.send(ClientMsg::Join(id_game)).await.unwrap();
            });
        } else {
            log::error!("failed to parse game id");
        }
    };

    view! { cx,
        h2(class="title is-5") { "Join game" }

        hr

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
