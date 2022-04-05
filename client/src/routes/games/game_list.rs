//! Implementation of the [`GameListPage`].

use crate::{
    components::{ErrorMsg, Leaderboard},
    services::games::{list, overall_stats},
};
use sycamore::prelude::*;

/// Page for overall user stats and a game list.
#[component]
pub fn GameListPage<G: Html>(ctx: ScopeRef) -> View<G> {
    view! { ctx,
        p { "TODO" }
    }
}
