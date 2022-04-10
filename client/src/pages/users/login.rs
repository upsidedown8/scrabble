//! Implementation of the [`LoginPage`].

use crate::{
    components::{ErrorMsg, Progress},
    context::{use_auth, AuthCtx},
    requests::users::login,
};
use api::routes::users::Login;
use sycamore::{futures::spawn_local_scoped, prelude::*};
use sycamore_router::navigate;

/// Page for signing in to an account.
#[component]
pub fn LoginPage<G: Html>(cx: Scope) -> View<G> {
    let auth = use_auth(cx);

    // create signals for the inputs.
    let username = create_signal(cx, String::new());
    let password = create_signal(cx, String::new());

    // state signals
    let is_loading = create_signal(cx, false);
    let err = create_signal(cx, None);

    // called when a user clicks the log in button.
    let on_log_in = move |_| {
        is_loading.set(true);
        err.set(None);

        spawn_local_scoped(cx, async {
            let req = Login {
                username: (*username.get()).clone(),
                password: (*password.get()).clone(),
            };

            match login(&req).await {
                Ok((token, user_details)) => {
                    auth.set(Some(AuthCtx {
                        token,
                        user_details,
                    }));
                    navigate("/");
                }
                Err(e) => err.set(Some(e)),
            }

            is_loading.set(false);
        });
    };

    view! { cx,
        div(class="page is-centered is-vcentered is-flex columns") {
            div(class="box") {
                div(class="has-text-centered") {
                    a(href="/users/signup") { "Need an account?" }
                    br
                    br
                    a(href="/users/reset-password") { "Forgot password?" }
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

                button(on:click=on_log_in, disabled=*is_loading.get(), class="button is-primary") {
                    "Log in"
                }

                Progress {
                    is_visible: is_loading,
                }
                ErrorMsg {
                    err: err,
                }
            }
        }
    }
}
