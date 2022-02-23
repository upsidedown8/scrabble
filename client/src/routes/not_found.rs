use sycamore::prelude::*;

#[component]
pub fn NotFoundPage<G: Html>(ctx: ScopeRef) -> View<G> {
    view! { ctx,
        div(class="not-found-route is-centered is-vcentered columns") {
            div {
                h1 {
                    "404: Not Found"
                }
                p {
                    "An error occured: the current URL does not refer to a route"
                }
            }
        }
    }
}
