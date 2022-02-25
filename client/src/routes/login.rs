//! Implementation of the [`LoginPage`].

use crate::{
    components::{ErrorMsg, ProgressBar},
    contexts::{AuthCtx, ScopeExt},
    services::users::login,
};
use api::users::Login;
use sycamore::{futures::ScopeSpawnFuture, prelude::*};
use sycamore_router::navigate;

/// Page for signing in to an account.
#[component]
pub fn LoginPage<G: Html>(ctx: ScopeRef) -> View<G> {
    let auth_ctx = ctx.use_auth_context();
    let username = ctx.create_signal(String::from(""));
    let password = ctx.create_signal(String::from(""));
    let login_req = ctx.create_memo(|| Login {
        username: (*username.get()).clone(),
        password: (*password.get()).clone(),
    });
    let loading = ctx.create_signal(false);
    let err = ctx.create_signal(None);

    let onsignin = |_| {
        loading.set(true);

        ctx.spawn_future(async {
            match login(auth_ctx, login_req.get().as_ref()).await {
                Ok(login_response) => {
                    auth_ctx.set(Some(AuthCtx {
                        details: login_response.user_details,
                        auth: login_response.auth,
                    }));

                    navigate("/");
                }
                Err(e) => err.set(Some(e)),
            };

            loading.set(false);
        });
    };

    view! { ctx,
        div(class="login-route is-centered is-vcentered is-flex columns") {
            div(class="box") {
                div(class="has-text-centered") {
                    a(href="/signup") {
                        "Need an account?"
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

                button(on:click=onsignin, disabled=*loading.get(), class="button is-primary") {
                    "Sign in"
                }

                ProgressBar(loading)
                ErrorMsg(err)
            }
        }
    }
}
