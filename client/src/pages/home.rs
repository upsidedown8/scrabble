//! Implementation of the [`HomePage`].

use sycamore::prelude::*;

/// The landing page.
#[component]
pub fn HomePage<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div(class="page") {
            h1 { "Home page" }
        }
    }
}
