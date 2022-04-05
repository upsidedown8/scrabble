//! Implementation of the [`SignUpPage`].

use crate::{
    components::{ErrorMsg, ProgressBar},
    contexts::{AuthCtx, ScopeExt},
    services::users::sign_up,
};
use api::routes::users::SignUp;
use sycamore::{futures::ScopeSpawnLocal, prelude::*};
use sycamore_router::navigate;

/// Page for creating an account.
#[component]
pub fn SignUpPage<G: Html>(ctx: ScopeRef) -> View<G> {
    let auth_ctx = ctx.use_auth_context();

    // Signals for signup options.
    let username = ctx.create_signal(String::from(""));
    let email = ctx.create_signal(String::from(""));
    let password = ctx.create_signal(String::from(""));
    let is_private = ctx.create_signal(false);

    // Request signal.
    let sign_up_req = ctx.create_memo(|| SignUp {
        username: (*username.get()).clone(),
        email: (*email.get()).clone(),
        password: (*password.get()).clone(),
        is_private: *is_private.get(),
    });

    // State signals.
    let loading = ctx.create_signal(false);
    let err = ctx.create_signal(None);

    // Called when the user clicks the signup button.
    let on_sign_up = |_| {
        loading.set(true);
        err.set(None);

        ctx.spawn_local(async {
            match sign_up(sign_up_req.get().as_ref()).await {
                // Successful request.
                Ok((auth, user_details)) => {
                    auth_ctx.set(Some(AuthCtx { user_details, auth }));
                    navigate("/");
                }
                // An error occurred.
                Err(e) => err.set(Some(e)),
            };

            loading.set(false);
        });
    };

    view! { ctx,
        div(class="page is-centered is-vcentered is-flex columns") {
            div(class="box") {
                div(class="has-text-centered") {
                    a(href="/login") {
                        "Already have an account?"
                    }
                }

                hr

                div(class="field") {
                    label(class="label") {
                        "Username"
                    }
                    div(class="control") {
                        input(type="text", class="input", placeholder="username", bind:value=username)
                    }
                }

                div(class="field") {
                    label(class="label") {
                        "Email"
                    }
                    div(class="control") {
                        input(type="email", class="input", placeholder="you@example.com", bind:value=email)
                    }
                }

                div(class="field") {
                    label(class="label") {
                        "Password"
                    }
                    div(class="control") {
                        input(type="password", class="input", placeholder="**********", bind:value=password)
                    }
                }

                div(class="field") {
                    label(class="checkbox") {
                        input(type="checkbox", checked=*is_private.get(), bind:checked=is_private)
                        "  Private account?"
                    }
                }

                div(class="field") {
                    button(on:click=on_sign_up, disabled=*loading.get(), class="button is-primary") {
                        "Sign up"
                    }
                }

                ProgressBar(loading)
                ErrorMsg(err)
            }
        }
    }
}
