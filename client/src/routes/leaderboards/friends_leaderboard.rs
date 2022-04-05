//! Implementation of the [`FriendsLeaderboardPage`].

use crate::{
    components::{ErrorMsg, Leaderboard},
    services::leaderboard::friends_leaderboard,
};
use api::routes::leaderboard::LeaderboardResponse;
use sycamore::{futures::ScopeSpawnLocal, prelude::*};

/// Page for the friends leaderboard.
#[component]
pub fn FriendsLeaderboardPage<G: Html>(ctx: ScopeRef) -> View<G> {
    // State signals
    let leaderboard = ctx.create_signal(vec![]);
    let err = ctx.create_signal(None);

    // Attempt to load the leaderboard.
    ctx.spawn_local(async {
        match friends_leaderboard().await {
            Ok(LeaderboardResponse { rows }) => leaderboard.set(rows),
            Err(e) => err.set(Some(e)),
        }
    });

    view! { ctx,
        div(class="page is-centered") {
            div {
                div(class="m-3 has-text-centered") {
                    h1(class="h1 is-size-5") { "Friends Leaderboard" }
                }

                Leaderboard(leaderboard)
                ErrorMsg(err)
            }
        }
    }
}
