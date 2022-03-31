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
    let username = ctx.create_signal(String::from(""));
    let email = ctx.create_signal(String::from(""));
    let password = ctx.create_signal(String::from(""));
    let sign_up_req = ctx.create_memo(|| SignUp {
        username: (*username.get()).clone(),
        email: (*email.get()).clone(),
        password: (*password.get()).clone(),
        is_private: false,
    });
    let loading = ctx.create_signal(false);
    let err = ctx.create_signal(None);

    let onsignup = |_| {
        loading.set(true);

        ctx.spawn_local(async {
            match sign_up(auth_ctx, sign_up_req.get().as_ref()).await {
                Ok(sign_up_response) => {
                    auth_ctx.set(Some(AuthCtx {
                        details: sign_up_response.user_details,
                        auth: sign_up_response.auth,
                    }));

                    navigate("/");
                }
                Err(e) => err.set(Some(e)),
            };

            loading.set(false);
        });
    };

    view! { ctx,
        div(class="signup-route is-centered is-vcentered is-flex columns") {
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

                button(on:click=onsignup, disabled=*loading.get(), class="button is-primary") {
                    "Sign up"
                }

                ProgressBar(loading)
                ErrorMsg(err)
            }
        }
    }
}
