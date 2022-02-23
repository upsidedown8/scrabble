use crate::components::{FaIcon, Separator};
use sycamore::prelude::*;

/// Appears at the bottom of every page.
#[component]
pub fn Footer<G: Html>(ctx: ScopeRef) -> View<G> {
    view! { ctx,
        footer(class="footer") {
            div(class="content has-text-centered") {
                p {
                    "Tom Thorogood"
                    Separator()
                    "Scrabble AI"
                    Separator()
                    a(href="https://github.com/upsidedown8/scrabble") {
                        "Code"
                        FaIcon("fa-brands fa-github")
                    }
                }
            }
        }
    }
}
