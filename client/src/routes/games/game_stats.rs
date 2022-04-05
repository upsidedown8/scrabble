//! Implementation of the [`GameStatsPage`].

use crate::{
    components::{ErrorMsg, Leaderboard},
    services::games::stats,
};
use sycamore::prelude::*;

/// Page for stats on a specific game.
#[component]
pub fn GameStatsPage<G: Html>(ctx: ScopeRef, id_game: i32) -> View<G> {
    view! { ctx,
        p { "TODO" }
    }
}
