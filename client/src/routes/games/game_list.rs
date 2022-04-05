//! Implementation of the [`GameListPage`].

use sycamore::prelude::*;

/// Page for overall user stats and a game list.
#[component]
pub fn GameListPage<G: Html>(ctx: ScopeRef) -> View<G> {
    view! { ctx,
        p { "TODO" }
    }
}
