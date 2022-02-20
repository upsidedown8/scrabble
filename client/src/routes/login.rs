use sycamore::prelude::*;

#[component]
pub fn LoginPage<G: Html>(ctx: ScopeRef) -> View<G> {
    view! { ctx,
        div(class="login-route") {
            h1 {
                "Login"
            }
        }
    }
}
