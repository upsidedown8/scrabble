//! Implementation of the [`LeaderboardPage`].

use crate::{
    components::{Leaderboard, StaticErrorMsg},
    context::use_auth,
    requests::leaderboard::friends_leaderboard,
};
use sycamore::{prelude::*, suspense::Suspense};

/// Page for the friends leaderboard.
#[component]
pub fn FriendsLeaderboardPage<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div(class="page") {
            section(class="is-centered") {
                div {
                    div(class="m-3 has-text-centered") {
                        h1(class="h1 is-size-5") { "Friends Leaderboard" }
                    }

                    Suspense {
                        fallback: view! { cx, p { "loading" } },
                        FetchFriendsLeaderboard {}
                    }
                }
            }
        }
    }
}

/// Component that makes an API request to display the leaderboard.
#[component]
async fn FetchFriendsLeaderboard<G: Html>(cx: Scope<'_>) -> View<G> {
    let auth = use_auth(cx);

    match friends_leaderboard(auth).await {
        Ok(response) => {
            let rows = create_signal(cx, response.rows);
            view! { cx,
                Leaderboard {
                    rows: rows,
                }
            }
        }
        Err(e) => view! { cx,
            StaticErrorMsg {
                err: e,
            }
        },
    }
}
