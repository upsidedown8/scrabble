//! A dot to seperate text nodes.

use sycamore::prelude::*;

/// Convenience component for a dot / " • " separator string.
#[component]
pub fn Separator<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        " • "
    }
}
