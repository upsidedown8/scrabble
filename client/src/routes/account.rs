//! Implementation of the [`AccountPage`].

use crate::contexts::{AuthCtx, ScopeExt};
use sycamore::prelude::*;

/// Page for managing user accounts.
#[component]
pub fn AccountPage<G: Html>(ctx: ScopeRef) -> View<G> {
    let auth_ctx = ctx.use_auth_context();

    let (username, email) = match auth_ctx.get().as_ref() {
        Some(AuthCtx { details, .. }) => (details.username.clone(), details.email.clone()),
        None => (String::new(), String::new()),
    };

    let username = ctx.create_signal(username);
    let email = ctx.create_signal(email);
    let password = ctx.create_signal(String::new());

    view! { ctx,
        div(class="account-route is-flex is-centered is-vcentered columns") {
            div(class="box") {
                div(class="field") {
                    label(class="label") {
                        "Username"
                    }
                    input(class="input", type="text", bind:value=username)
                }

                div(class="field") {
                    label(class="label") {
                        "Email"
                    }
                    input(class="input", type="email", bind:value=email)
                }

                hr

                div(class="field") {
                    label(class="label") {
                        "Password"
                    }
                    input(class="input", type="password", bind:value=password)
                }
            }
        }
    }
}
