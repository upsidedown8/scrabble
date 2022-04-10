//! The footer component.

use crate::components::{FaIcon, Separator};
use sycamore::prelude::*;

/// Appears at the bottom of every page.
#[component]
pub fn Footer<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        footer(class="footer") {
            div(class="content has-text-centered") {
                p {
                    "Tom Thorogood"
                    Separator {}
                    "Scrabble AI"
                    Separator {}
                    a(href="https://github.com/upsidedown8/scrabble") {
                        "Code"
                        FaIcon {
                            class: "fa-brands fa-github"
                        }
                    }
                }
            }
        }
    }
}
