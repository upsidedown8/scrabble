//! Implementation of the [`NotFoundPage`].

use sycamore::prelude::*;

/// Page rendered when the url did not match any other page,
/// or the page matched required the user to be logged in.
#[component]
pub fn NotFoundPage<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div(class="page") {
            section(class="is-centered is-vcentered columns") {
                div(class="has-text-centered") {
                    h1(class="is-size-5") {
                        "Not Found"
                    }

                    p {
                        "An error occured: the current URL does not refer to a route"
                    }
                }
            }
        }
    }
}
