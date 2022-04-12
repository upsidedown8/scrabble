//! Implementation of the [`FriendsPage`].

use crate::{
    components::{ErrorMsg, FriendsTable, StaticErrorMsg},
    context::use_auth,
    requests::friends,
};
use sycamore::{futures::spawn_local_scoped, prelude::*, suspense::Suspense};
use sycamore_router::navigate;

/// Page for managing friends and friend requests.
#[component]
pub fn FriendsPage<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div(class="page is-centered") {
            div {
                div(class="m-3 has-text-centered") {
                    h1(class="h1 is-size-5") { "Friends" }
                }

                section {
                    ManageFriend {}
                }

                section {
                    h1 { "Sent friend requests" }

                    Suspense {
                        fallback: view! { cx, p { "Loading sent friend requests" } },
                        SentFriendRequests {}
                    }
                }

                section {
                    h1 { "Received friend requests" }

                    Suspense {
                        fallback: view! { cx, p { "Loading received friend requests" } },
                        ReceivedFriendRequests {}
                    }
                }

                section {
                    h1 { "Friends" }

                    Suspense {
                        fallback: view! { cx, p { "Loading friends" } },
                        Friends {}
                    }
                }
            }
        }
    }
}

#[component]
fn ManageFriend<G: Html>(cx: Scope) -> View<G> {
    let auth = use_auth(cx);

    // input signals
    let username = create_signal(cx, String::new());

    // state signals
    let is_loading = create_signal(cx, false);
    let err = create_signal(cx, None);

    // called when the remove button is clicked.
    let on_remove = move |_| {
        log::trace!("removing friend");
        is_loading.set(true);
        err.set(None);

        spawn_local_scoped(cx, async {
            let username = (*username.get()).clone();

            match friends::remove(auth, username).await {
                Ok(()) => navigate("/friends"),
                Err(e) => {
                    err.set(Some(e));
                    is_loading.set(false);
                }
            }
        });
    };

    // called when the send request button is clicked.
    let on_make_request = move |_| {
        log::trace!("sending friend request");
        is_loading.set(true);
        err.set(None);

        spawn_local_scoped(cx, async {
            let username = (*username.get()).clone();

            match friends::add(auth, username).await {
                Ok(()) => navigate("/friends"),
                Err(e) => {
                    err.set(Some(e));
                    is_loading.set(false);
                }
            }
        });
    };

    view! { cx,
        div(class="box mx-4 mt-2") {
            label(class="label") { "Username" }
            div(class="field has-addons") {
                div(class="control is-expanded") {
                    input(
                        class="input",
                        type="text",
                        placeholder="Username",
                        bind:value=username,
                    )
                }
                div(class="control") {
                    button(class="button is-danger", on:click=on_remove) {
                        "Remove friend"
                    }
                }
                div(class="control") {
                    button(class="button is-primary", on:click=on_make_request) {
                        "Send request"
                    }
                }
            }

            ErrorMsg {
                err: err,
            }
        }
    }
}

/// Component that fetches a sent friend requests.
#[component]
async fn SentFriendRequests<G: Html>(cx: Scope<'_>) -> View<G> {
    match friends::list_outgoing(use_auth(cx)).await {
        Ok(response) => view! { cx,
            FriendsTable {
                from_header: "Sent to".to_string(),
                date_header: "Date".to_string(),
                rows: response.requests,
            }
        },
        Err(e) => view! { cx,
            StaticErrorMsg {
                err: e,
            }
        },
    }
}

/// Component that fetches received friend requests.
#[component]
async fn ReceivedFriendRequests<G: Html>(cx: Scope<'_>) -> View<G> {
    match friends::list_incoming(use_auth(cx)).await {
        Ok(response) => view! { cx,
            FriendsTable {
                from_header: "Sent from".to_string(),
                date_header: "Date".to_string(),
                rows: response.requests,
            }
        },
        Err(e) => view! { cx,
            StaticErrorMsg {
                err: e,
            }
        },
    }
}

/// Component that fetches a user's friends.
#[component]
async fn Friends<G: Html>(cx: Scope<'_>) -> View<G> {
    match friends::list(use_auth(cx)).await {
        Ok(response) => view! { cx,
            FriendsTable {
                from_header: "Username".to_string(),
                date_header: "Friends since".to_string(),
                rows: response.friends,
            }
        },
        Err(e) => view! { cx,
            StaticErrorMsg {
                err: e,
            }
        },
    }
}
