//! Page that allows users to send an email to reset their password.

use sycamore::prelude::*;

/// ResetPassword page.
#[component]
pub fn ResetPasswordPage<G: Html>(ctx: ScopeRef) -> View<G> {
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
