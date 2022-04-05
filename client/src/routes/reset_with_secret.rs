//! Page that allows users to update their password.

use sycamore::prelude::*;

#[derive(Prop)]
pub struct Props {
    pub username: String,
    pub secret: String,
}

/// ResetWithSecret page.
#[component]
pub fn ResetWithSecretPage<G: Html>(ctx: ScopeRef, props: Props) -> View<G> {
    let password = ctx.create_signal(String::new());

    view! { ctx,
        div(class="page is-centered is-vcentered is-flex columns") {
            div(class="box") {
                h1 { "Reset password" }

                hr

                div(class="field") {
                    label(class="label") {
                        "New password"
                    }
                    div(class="control") {
                        input(type="password", class="input", placeholder="**********", bind:value=password)
                    }
                }
            }
        }
    }
}
