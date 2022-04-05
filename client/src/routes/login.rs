//! Implementation of the [`LoginPage`].

use crate::{
    components::{ErrorMsg, ProgressBar},
    contexts::{AuthCtx, ScopeExt},
    services::users::login,
};
use api::routes::users::Login;
use sycamore::{futures::ScopeSpawnLocal, prelude::*};
use sycamore_router::navigate;

/// Page for signing in to an account.
#[component]
pub fn LoginPage<G: Html>(ctx: ScopeRef) -> View<G> {
    let auth_ctx = ctx.use_auth_context();

    // Input signals
    let username = ctx.create_signal(String::from(""));
    let password = ctx.create_signal(String::from(""));

    // State signals
    let loading = ctx.create_signal(false);
    let err = ctx.create_signal(None);

    // Called when the user clicks the sign in button.
    let on_sign_in = |_| {
        loading.set(true);

        ctx.spawn_local(async {
            let req = Login {
                username: (*username.get()).clone(),
                password: (*password.get()).clone(),
            };

            match login(&req).await {
                // Successful request.
                Ok((auth, user_details)) => {
                    auth_ctx.set(Some(AuthCtx { user_details, auth }));
                    navigate("/");
                }
                // An error occured.
                Err(e) => err.set(Some(e)),
            };

            loading.set(false);
        });
    };

    view! { ctx,
        div(class="page is-centered is-vcentered is-flex columns") {
            div(class="box") {
                div(class="has-text-centered") {
                    p (class="mb-3") {
                        a(href="/signup") {
                            "Need an account?"
                        }
                    }

                    p {
                        a(href="/reset-password") {
                            "Forgot password?"
                        }
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
                        "Password"
                    }
                    div(class="control") {
                        input(type="password", class="input", placeholder="**********", bind:value=password)
                    }
                }

                button(on:click=on_sign_in, disabled=*loading.get(), class="button is-primary") {
                    "Sign in"
                }

                ProgressBar(loading)
                ErrorMsg(err)
            }
        }
    }
}
