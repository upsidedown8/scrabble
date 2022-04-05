//! Page that allows users to send an email to reset their password.

use crate::{
    components::{ErrorMsg, ProgressBar},
    services::users::reset_password,
};
use api::routes::users::ResetPassword;
use sycamore::{futures::ScopeSpawnLocal, prelude::*};

/// ResetPassword page.
#[component]
pub fn ResetPasswordPage<G: Html>(ctx: ScopeRef) -> View<G> {
    // Input signal
    let username = ctx.create_signal(String::new());

    // State signals
    let loading = ctx.create_signal(false);
    let err = ctx.create_signal(None);
    let success = ctx.create_signal(false);

    // Called when the user clicks the reset button.
    let on_reset = |_| {
        loading.set(true);
        err.set(None);

        ctx.spawn_local(async {
            let req = ResetPassword {
                username: (*username.get()).clone(),
            };

            match reset_password(&req).await {
                Ok(()) => {}
                Err(e) => err.set(Some(e)),
            }

            loading.set(false);
        })
    };

    view! { ctx,
        div(class="page is-centered is-vcentered is-flex columns") {
            div(class="box") {
                div(class="field") {
                    label(class="label") {
                        "Username"
                    }
                    div(class="control") {
                        input(type="text", class="input", placeholder="Username", bind:value=username)
                    }
                }

                button(on:click=on_reset, disabled=*loading.get(), class="button is-primary") {
                    "Reset password"
                }

                ProgressBar(loading)
                ErrorMsg(err)

                (if *success.get() {
                    view! { ctx,
                        p {
                            "Request succeeded. Check your email."
                        }
                    }
                } else {
                    view! { ctx, }
                })
            }
        }
    }
}
