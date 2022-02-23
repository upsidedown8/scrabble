use sycamore::prelude::*;

#[component]
pub fn Separator<G: Html>(ctx: ScopeRef) -> View<G> {
    view! { ctx,
        " • "
    }
}
