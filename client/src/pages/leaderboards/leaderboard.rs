//! Implementation of the [`LeaderboardPage`].

use crate::{
    components::{Leaderboard, StaticErrorMsg},
    requests::leaderboard::overall_leaderboard,
};
use api::routes::leaderboard::LeaderboardResponse;
use sycamore::{prelude::*, suspense::Suspense};

/// Page for the overall leaderboard.
#[component]
pub fn LeaderboardPage<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div(class="page") {
            section(class="is-centered") {
                div {
                    div(class="m-3 has-text-centered") {
                        h1(class="h1 is-size-5") { "Leaderboard" }
                    }

                    Suspense {
                        fallback: view! { cx, p { "loading" } },
                        FetchLeaderboard {}
                    }
                }
            }
        }
    }
}

/// Component that makes an API request to display the leaderboard.
#[component]
async fn FetchLeaderboard<G: Html>(cx: Scope<'_>) -> View<G> {
    match overall_leaderboard(15, 0).await {
        Ok(response) => view! { cx,
            ViewLeaderboard(response)
        },
        Err(e) => view! { cx,
            StaticErrorMsg {
                err: e,
            }
        },
    }
}

/// Component that displays the leaderboard.
#[component]
fn ViewLeaderboard<G: Html>(cx: Scope, response: LeaderboardResponse) -> View<G> {
    let rows = create_signal(cx, response.rows);

    view! { cx,
        Leaderboard {
            rows: rows,
        }
    }
}
