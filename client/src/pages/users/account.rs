//! Implementation of the [`AccountPage`].

use crate::{
    components::{ErrorMsg, Progress},
    context::{set_user_details, use_auth, use_user_details},
    requests::users::{delete, update},
};
use api::routes::users::{DeleteAccount, UpdateAccount, UserDetails};
use sycamore::{futures::spawn_local_scoped, prelude::*};
use sycamore_router::navigate;

/// Page for managing user accounts,
#[component]
pub fn AccountPage<G: Html>(cx: Scope) -> View<G> {
    let auth = use_auth(cx);

    // get the current user information.
    let details = {
        let hook = use_user_details(cx).get();
        hook.as_ref().clone().unwrap()
    };

    // create signals for the inputs.
    let username = create_signal(cx, details.username);
    let email = create_signal(cx, details.email);
    let is_private = create_signal(cx, details.is_private);
    let password = create_signal(cx, String::new());
    let curr_password = create_signal(cx, String::new());

    // state signals
    let is_loading = create_signal(cx, false);
    let err = create_signal(cx, None);

    // called when a user clicks the delete button.
    let on_delete = move |_| {
        log::trace!("deleting account");

        is_loading.set(true);
        err.set(None);

        spawn_local_scoped(cx, async {
            let req = DeleteAccount {
                password: (*curr_password.get()).clone(),
            };

            match delete(auth, &req).await {
                Ok(()) => {
                    auth.set(None);
                    navigate("/users/account");
                }
                Err(e) => {
                    is_loading.set(false);
                    err.set(Some(e))
                }
            }
        });
    };

    // called when a user clicks the update button.
    let on_update = move |_| {
        log::trace!("updating account");

        is_loading.set(true);
        err.set(None);

        spawn_local_scoped(cx, async {
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

            match update(auth, &req).await {
                Ok(()) => {
                    log::info!("update succeeded");
                    set_user_details(
                        auth,
                        UserDetails {
                            username: (*username.get()).clone(),
                            email: (*email.get()).clone(),
                            is_private: *is_private.get(),
                        },
                    );
                    navigate("/users/account");
                }
                Err(e) => {
                    is_loading.set(false);
                    err.set(Some(e))
                }
            }
        });
    };

    view! { cx,
        div(class="page") {
            section(class="is-fullheight is-flex is-centered is-vcentered columns") {
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

                    div(class="field is-grouped") {
                        div(class="control") {
                            button(on:click=on_update, disabled=*is_loading.get(), class="button is-primary") {
                                "Update"
                            }
                        }
                        div(class="control") {
                            button(on:click=on_delete, disabled=*is_loading.get(), class="button is-danger") {
                                "Delete"
                            }
                        }
                    }

                    Progress {
                        is_visible: is_loading
                    }
                    ErrorMsg {
                        err: err,
                    }
                }
            }
        }
    }
}
