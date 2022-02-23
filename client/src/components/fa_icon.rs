use sycamore::prelude::*;

#[component]
pub fn FaIcon<'a, G: Html>(ctx: ScopeRef<'a>, class: &'a str) -> View<G> {
    view! { ctx,
        span(class="mx-1") {
            i(class=class)
        }
    }
}
