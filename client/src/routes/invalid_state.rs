//! Implementation of the [`InvalidStatePage`].

use sycamore::prelude::*;

/// Page rendered when the page is not possible to render in the
/// current state (e.g. the login page when the user is signed in).
#[component]
pub fn InvalidStatePage<G: Html>(ctx: ScopeRef) -> View<G> {
    view! { ctx,
        div(class="page is-centered is-vcentered columns") {
            div(class="has-text-centered") {
                h1(class="h1 is-size-5") {
                    "Invalid state"
                }

                p {
                    "An error occured: the current page cannot be displayed in this state."
                    "Perhaps you need to log in?"
                }
            }
        }
    }
}
