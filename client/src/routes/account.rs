use api::users::DeleteUser;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::use_async;

use crate::{
    contexts::{get_token, use_auth_context},
    services::users,
};

#[function_component(AccountRoute)]
pub fn account_route() -> Html {
    let auth_ctx = use_auth_context();
    let password_ref = use_node_ref();

    let delete_async = {
        let auth_ctx = auth_ctx.clone();
        let password_ref = password_ref.clone();

        use_async(async move {
            let password = password_ref.cast::<HtmlInputElement>().unwrap().value();
            let user_delete = DeleteUser { password };

            let res = users::delete(&user_delete)
                .await
                .map_err(|_| String::from("error"));

            auth_ctx.logout();

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

    html! {
        <div class="account-route">
            <h1>{ "The account route" }</h1>

            <p>{ "username: " }{ &auth_ctx.username }</p>
            <p>{ "email: " }{ &auth_ctx.email }</p>

            if let Some(t) = get_token() {
                <p>{ "token: " }{ t }</p>
            }

            <div class="box">
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

                <a class="button is-danger" onclick={ondelete}>
                    { "Delete account" }
                </a>
            </div>
        </div>
    }
}
