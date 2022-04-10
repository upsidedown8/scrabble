//! Implementation of the [`ResetPasswordPage`].

use crate::{
    components::{ErrorMsg, Progress},
    requests::users::reset_password,
};
use api::routes::users::ResetPassword;
use sycamore::{futures::spawn_local_scoped, prelude::*};

/// ResetPassword page.
#[component]
pub fn ResetPasswordPage<G: Html>(cx: Scope) -> View<G> {
    // input signal
    let username = create_signal(cx, String::new());

    // state signals
    let is_loading = create_signal(cx, false);
    let is_success = create_signal(cx, false);
    let err = create_signal(cx, None);

    // called when a user clicks the reset button.
    let on_reset = move |_| {
        is_loading.set(true);
        err.set(None);

        spawn_local_scoped(cx, async {
            let req = ResetPassword {
                username: (*username.get()).clone(),
            };

            match reset_password(&req).await {
                Ok(()) => is_success.set(true),
                Err(e) => err.set(Some(e)),
            }

            is_loading.set(false);
        });
    };

    view! { cx,
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

                button(on:click=on_reset, disabled=*is_loading.get(), class="button is-primary") {
                    "Reset password"
                }

                Progress {
                    is_visible: is_loading,
                }
                ErrorMsg {
                    err: err,
                }

                (match *is_success.get() {
                    false => view! { cx, },
                    true => view! { cx,
                        p {
                            "Request succeeded. Check your email."
                        }
                    },
                })
            }
        }
    }
}
