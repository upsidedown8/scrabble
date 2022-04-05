//! Implementation of the [`NotFoundPage`].

use sycamore::prelude::*;

/// Page rendered when the url did not match any other page,
/// or the page matched required the user to be logged in.
#[component]
pub fn NotFoundPage<G: Html>(ctx: ScopeRef) -> View<G> {
    view! { ctx,
        div(class="page is-centered is-vcentered columns") {
            div {
                h1(class="h1") {
                    "404: Not Found"
                }
                p {
                    "An error occured: the current URL does not refer to a route"
                }
            }
        }
    }
}
