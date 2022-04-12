//! Implementation of the [`LoginPage`].

use crate::{
    components::{ErrorMsg, Progress},
    context::{use_auth, AuthCtx},
    requests::users::reset_with_secret,
};
use api::routes::users::ResetWithSecret;
use sycamore::{futures::spawn_local_scoped, prelude::*};
use sycamore_router::navigate;

/// Props for `ResetWithSecretPage`.
#[derive(Prop)]
pub struct Props {
    /// The username of the account.
    pub username: String,
    /// The secret to reset the password.
    pub secret: String,
}

/// Page for signing in to an account.
#[component]
pub fn ResetWithSecretPage<G: Html>(cx: Scope, props: Props) -> View<G> {
    let auth = use_auth(cx);

    // input signal
    let password = create_signal(cx, String::new());

    // state signals
    let props = create_signal(cx, props);
    let is_loading = create_signal(cx, false);
    let err = create_signal(cx, None);

    // called when the user resets their password.
    let on_reset = move |_| {
        log::trace!("resetting password");

        is_loading.set(true);
        err.set(None);

        spawn_local_scoped(cx, async {
            let props = props.get();
            let req = ResetWithSecret {
                secret_hex: props.secret.clone(),
                new_password: (*password.get()).clone(),
                username: props.username.clone(),
            };

            match reset_with_secret(&req).await {
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

                button(on:click=on_reset, disabled=*is_loading.get(), class="button is-primary") {
                    "Update password"
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
