use api::users::DeleteAccount;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::{use_async, use_bool_toggle};

use crate::{contexts::use_auth_context, services::users};

#[function_component(AccountRoute)]
pub fn account_route() -> Html {
    let auth_ctx = use_auth_context();
    let password_ref = use_node_ref();
    let incorrect_pass = use_bool_toggle(false);

    let delete_async = {
        let incorrect_pass = incorrect_pass.clone();
        let auth_ctx = auth_ctx.clone();
        let password_ref = password_ref.clone();

        use_async(async move {
            let password = password_ref.cast::<HtmlInputElement>().unwrap().value();
            let user_delete = DeleteAccount { password };

            let res = users::delete(&user_delete)
                .await
                .map_err(|_| String::from("error"));

            incorrect_pass.set(res.is_err());

            if res.is_ok() {
                auth_ctx.logout();
            }

            res
        })
    };

    let ondelete = {
        let delete_async = delete_async.clone();

        Callback::from(move |_| {
            let delete_async = delete_async.clone();

            delete_async.run();
        })
    };
    let onclick_delete = {
        let incorrect_pass = incorrect_pass.clone();

        Callback::from(move |_| incorrect_pass.set(false))
    };

    html! {
        <div class="account-route columns is-flex is-vcentered is-centered">
            <div class="box">
                <p>{ "username: " }{ &auth_ctx.username }</p>
                <p>{ "email: " }{ &auth_ctx.email }</p>

                <div class="field">
                    <label class="label">{ "Password" }</label>
                    <div class="control">
                        <input
                            class="input"
                            type="password"
                            placeholder="**********"
                            ref={password_ref.clone()}
                        />
                    </div>
                </div>

                <a disabled={delete_async.loading} class="button is-danger" onclick={ondelete}>
                    { "Delete account" }
                </a>

                if delete_async.loading {
                    <progress class="progress is-small is-primary my-2" max="100">
                        { "10%" }
                    </progress>
                } else if *incorrect_pass {
                    <div class="notification is-danger my-2">
                        <button onclick={onclick_delete} class="delete" />
                        { "Incorrect password" }
                    </div>
                }
            </div>
        </div>
    }
}
