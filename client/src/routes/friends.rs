//! Implementation of the [`FriendsPage`].

use sycamore::prelude::*;

/// Page for managing friends and friend requests.
#[component]
pub fn FriendsPage<G: Html>(ctx: ScopeRef) -> View<G> {
    view! { ctx,
        p { "TODO" }
    }
}
