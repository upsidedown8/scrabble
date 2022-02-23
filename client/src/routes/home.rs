//! Implementation of the [`HomePage`].

use sycamore::prelude::*;

/// The landing page.
#[component]
pub fn HomePage<G: Html>(ctx: ScopeRef) -> View<G> {
    view! { ctx,
        div(class="home-route") {

        }
    }
}
