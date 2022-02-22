use sycamore::prelude::*;

#[component]
pub fn Footer<G: Html>(ctx: ScopeRef) -> View<G> {
    view! { ctx,
        footer(class="footer") {
            div(class="content has-text-centered") {
                p {
                    "Scrabble AI"
                    a(href="https://github.com/upsidedown8/scrabble") {
                        span(class="icon") {
                            i(class="fa-brands fa-github")
                        }
                    }
                }                
            }
        }
    }
}
