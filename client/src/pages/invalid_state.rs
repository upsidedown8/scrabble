//! Implementation of the [`InvalidStatePage`].

use sycamore::prelude::*;

/// Page rendered when the page is not possible to render in the
/// current state (e.g. the login page when the user is signed in).
#[component]
pub fn InvalidStatePage<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div(class="page") {
            section(class="is-centered is-vcentered columns") {
                div(class="has-text-centered") {
                    h1(class="h1 is-size-5") {
                        "Invalid state"
                    }

                    p {
                        "An error occured: the current page cannot be displayed in this state."

                        br

                        "Perhaps you need to log in?"
                    }
                }
            }
        }
    }
}
