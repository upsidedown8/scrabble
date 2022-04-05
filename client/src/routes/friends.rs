//! Implementation of the [`FriendsPage`].

use crate::components::ErrorMsg;
use sycamore::prelude::*;

/// Page for managing friends and friend requests.
#[component]
pub fn FriendsPage<G: Html>(ctx: ScopeRef) -> View<G> {
    let err = ctx.create_signal(None);

    view! { ctx,
        div(class="page is-centered") {
            div {
                div(class="m-3 has-text-centered") {
                    h1(class="h1 is-size-5") { "Friends" }
                }

                ErrorMsg(err)
            }
        }
    }
}
