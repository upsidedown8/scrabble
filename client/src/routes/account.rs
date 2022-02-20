use sycamore::prelude::*;

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
