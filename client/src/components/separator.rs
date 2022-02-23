use sycamore::prelude::*;

/// Convenience component for a dot / " • " separator string.
#[component]
pub fn Separator<G: Html>(ctx: ScopeRef) -> View<G> {
    view! { ctx,
        " • "
    }
}
