//! Implementation of the [`AccountPage`].

use crate::{
    components::{ErrorMsg, ProgressBar},
    contexts::{AuthCtx, ScopeExt},
    services::users::update,
};
use api::routes::users::{UpdateAccount, UserDetails};
use sycamore::{futures::ScopeSpawnLocal, prelude::*};
use sycamore_router::navigate;

/// Page for managing user accounts.
#[component]
pub fn AccountPage<G: Html>(ctx: ScopeRef) -> View<G> {
    let auth_ctx = ctx.use_auth_context();

    let details = {
        let auth_ctx = ctx.use_auth_context().get();
        auth_ctx.as_ref().as_ref().unwrap().user_details.clone()
    };

    // Create signals for the username, email and password.
    let username = ctx.create_signal(details.username);
    let email = ctx.create_signal(details.email);
    let is_private = ctx.create_signal(details.is_private);
    let password = ctx.create_signal(String::new());
    let curr_password = ctx.create_signal(String::new());

    // State signals
    let loading = ctx.create_signal(false);
    let err = ctx.create_signal(None);

    // Called when the user clicks the update button.
    let on_update = |_| {
        loading.set(true);

        ctx.spawn_local(async {
            let req = UpdateAccount {
                old_password: (*curr_password.get()).clone(),
                email: Some((*email.get()).clone()),
                username: Some((*username.get()).clone()),
                password: match password.get().is_empty() {
                    true => None,
                    false => Some((*password.get()).clone()),
                },
                is_private: Some(*is_private.get()),
            };

            match update(auth_ctx, &req).await {
                Ok(()) => {
                    // Get the current auth token.
                    let auth = ctx.use_token().get();
                    let auth = auth.as_ref().as_ref().unwrap().clone();

                    // Update the auth info with the new data.
                    auth_ctx.set(Some(AuthCtx {
                        user_details: UserDetails {
                            username: (*username.get()).clone(),
                            email: (*email.get()).clone(),
                            is_private: *is_private.get(),
                        },
                        auth,
                    }));

                    // Navigate to the home page.
                    navigate("/");
                }
                Err(e) => err.set(Some(e)),
            }
        });
    };

    view! { ctx,
        div(class="page is-flex is-centered is-vcentered columns") {
            div(class="box") {
                div(class="field") {
                    label(class="label") {
                        "Username"
                    }
                    div(class="control") {
                        input(class="input", type="text", bind:value=username)
                    }
                }

                div(class="field") {
                    label(class="label") {
                        "Email"
                    }
                    div(class="control") {
                        input(class="input", type="email", bind:value=email)
                    }
                }

                div(class="field") {
                    label(class="label") {
                        "New password"
                    }
                    div(class="control") {
                        input(class="input", type="password", bind:value=password)
                    }
                }

                hr

                div(class="field") {
                    label(class="label") {
                        "Current password"
                    }
                    div(class="control") {
                        input(class="input", type="password", bind:value=curr_password)
                    }
                }

                hr

                button(on:click=on_update, disabled=*loading.get(), class="button is-primary") {
                    "Update"
                }

                ProgressBar(loading)
                ErrorMsg(err)
            }
        }
    }
}
