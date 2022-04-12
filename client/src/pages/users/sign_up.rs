//! Implementation of the [`SignUpPage`].

use crate::{
    components::{ErrorMsg, Progress},
    context::{use_auth, AuthCtx},
    requests::users::sign_up,
};
use api::routes::users::SignUp;
use sycamore::{futures::spawn_local_scoped, prelude::*};
use sycamore_router::navigate;

#[component]
pub fn SignUpPage<G: Html>(cx: Scope) -> View<G> {
    let auth = use_auth(cx);

    // input signals
    let username = create_signal(cx, String::new());
    let email = create_signal(cx, String::new());
    let password = create_signal(cx, String::new());
    let is_private = create_signal(cx, false);

    // state signals
    let is_loading = create_signal(cx, false);
    let err = create_signal(cx, None);

    // called when a user clicks the signup button.
    let on_sign_up = move |_| {
        log::trace!("signing up");

        is_loading.set(true);
        err.set(None);

        spawn_local_scoped(cx, async {
            let req = SignUp {
                username: (*username.get()).clone(),
                email: (*email.get()).clone(),
                password: (*password.get()).clone(),
                is_private: *is_private.get(),
            };

            match sign_up(&req).await {
                Ok((token, user_details)) => {
                    auth.set(Some(AuthCtx {
                        token,
                        user_details,
                    }));
                    navigate("/");
                }
                Err(e) => {
                    is_loading.set(false);
                    err.set(Some(e))
                }
            }
        });
    };

    view! { cx,
        div(class="page is-centered is-vcentered is-flex columns") {
            div(class="box") {
                div(class="has-text-centered") {
                    a(href="/users/login") {
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
                        input(type="checkbox", bind:checked=is_private)
                        "  Private account?"
                    }
                }

                div(class="field") {
                    button(on:click=on_sign_up, disabled=*is_loading.get(), class="button is-primary") {
                        "Sign up"
                    }
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
