//! Implementation of the [`AccountPage`].

use sycamore::prelude::*;

/// Page for managing user accounts.
#[component]
pub fn AccountPage<G: Html>(ctx: ScopeRef) -> View<G> {
    view! { ctx,
        div(class="account-route") {
            h1 {
                "Account"
            }
        }
    }
}
