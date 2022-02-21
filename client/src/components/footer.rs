use sycamore::prelude::*;

#[component]
pub fn Footer<G: Html>(ctx: ScopeRef) -> View<G> {
    view! { ctx,
        footer(class="footer") {
            "footer"
        }
    }
}
