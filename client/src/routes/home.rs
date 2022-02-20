use sycamore::prelude::*;

#[component]
pub fn HomePage<G: Html>(ctx: ScopeRef) -> View<G> {
    view! { ctx,
        div(class="home-route") {
            h1 {
                "Home"
            }
        }
    }
}
