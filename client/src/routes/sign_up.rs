use sycamore::prelude::*;

#[component]
pub fn SignUpPage<G: Html>(ctx: ScopeRef) -> View<G> {
    view! { ctx,
        div(class="signup-route") {
            h1 {
                "Sign up"
            }
        }
    }
}
