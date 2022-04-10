//! A FontAwesome icon.

use sycamore::prelude::*;

/// Properties for `FaIcon`.
#[derive(Prop)]
pub struct Props<'a> {
    /// The class of the icon.
    pub class: &'a str,
}

/// Component for consistent font-awesome icons. Applies a
/// margin to the left and right of the icon. The `class` prop
/// is used to set the icon.
#[component]
pub fn FaIcon<'a, G: Html>(cx: Scope<'a>, props: Props<'a>) -> View<G> {
    view! { cx,
        span(class="mx-1") {
            i(class=props.class)
        }
    }
}
