use sycamore::prelude::*;

/// Component for consistent font-awesome icons. Applies a
/// margin to the left and right of the icon. The `class` prop
/// is used to set the icon.
#[component]
pub fn FaIcon<'a, G: Html>(ctx: ScopeRef<'a>, class: &'a str) -> View<G> {
    view! { ctx,
        span(class="mx-1") {
            i(class=class)
        }
    }
}
