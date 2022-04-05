//! Implementation of the [`LeaderboardPage`].

use crate::{
    components::{ErrorMsg, Leaderboard},
    services::leaderboard::overall_leaderboard,
};
use api::routes::leaderboard::LeaderboardResponse;
use sycamore::{futures::ScopeSpawnLocal, prelude::*};

/// Page for the overall leaderboard.
#[component]
pub fn LeaderboardPage<G: Html>(ctx: ScopeRef) -> View<G> {
    // State signals
    let leaderboard = ctx.create_signal(vec![]);
    let err = ctx.create_signal(None);

    // Attempt to load the leaderboard.
    ctx.spawn_local(async {
        match overall_leaderboard(15, 0).await {
            Ok(LeaderboardResponse { rows }) => leaderboard.set(rows),
            Err(e) => err.set(Some(e)),
        }
    });

    view! { ctx,
        div(class="page is-centered") {
            div {
                div(class="m-3 has-text-centered") {
                    h1(class="h1 is-size-5") { "Leaderboard" }
                }

                Leaderboard(leaderboard)
                ErrorMsg(err)
            }
        }
    }
}
