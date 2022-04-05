//! Page that allows users to update their password.

use crate::{
    components::{ErrorMsg, ProgressBar},
    contexts::{AuthCtx, ScopeExt},
    services::users::reset_with_secret,
};
use api::routes::users::ResetWithSecret;
use sycamore::{futures::ScopeSpawnLocal, prelude::*};
use sycamore_router::navigate;

#[derive(Prop)]
pub struct Props {
    pub username: String,
    pub secret: String,
}

/// ResetWithSecret page.
#[component]
pub fn ResetWithSecretPage<G: Html>(ctx: ScopeRef, props: Props) -> View<G> {
    // Input signal
    let password = ctx.create_signal(String::new());
    let props = ctx.create_signal(props);

    // State signals
    let loading = ctx.create_signal(false);
    let err = ctx.create_signal(None);

    // Called when the user resets their password
    let on_reset = |_| {
        loading.set(true);
        err.set(None);

        ctx.spawn_local(async {
            let req = ResetWithSecret {
                secret_hex: props.get().secret.clone(),
                new_password: (*password.get()).clone(),
                username: props.get().username.clone(),
            };

            match reset_with_secret(&req).await {
                Ok((auth, user_details)) => {
                    // Update the token and navigate to the homepage.
                    ctx.use_auth_context()
                        .set(Some(AuthCtx { user_details, auth }));
                    navigate("/");
                }
                Err(e) => err.set(Some(e)),
            }

            loading.set(false);
        });
    };

    view! { ctx,
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

                button(on:click=on_reset, disabled=*loading.get(), class="button is-primary") {
                    "Update password"
                }

                ProgressBar(loading)
                ErrorMsg(err)
            }
        }
    }
}
