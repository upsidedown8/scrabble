use crate::{contexts::use_auth_context, services::users};
use api::users::Login;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::{use_async, use_bool_toggle};

#[function_component(LoginRoute)]
pub fn login_route() -> Html {
    let incorrect_details = use_bool_toggle(false);
    let auth_ctx = use_auth_context();
    let username_ref = use_node_ref();
    let password_ref = use_node_ref();

    let login_req = {
        let username_ref = username_ref.clone();
        let password_ref = password_ref.clone();
        let incorrect_details = incorrect_details.clone();

        use_async(async move {
            let username = username_ref.cast::<HtmlInputElement>().unwrap().value();
            let password = password_ref.cast::<HtmlInputElement>().unwrap().value();
            let user_login = Login { username, password };

            let res = users::login(&user_login)
                .await
                .map_err(|_| String::from("error"));

            incorrect_details.set(res.is_err());

            if let Ok(res) = &res {
                auth_ctx.login(res.clone())
            }

            res
        })
    };

    let onclick = {
        let login_req = login_req.clone();

        Callback::from(move |_| {
            let login_req = login_req.clone();

            login_req.run();
        })
    };
    let onclick_delete = {
        let incorrect_details = incorrect_details.clone();

        Callback::from(move |_| incorrect_details.toggle())
    };

    html! {
        <>
        <div class="login-route columns is-flex is-vcentered is-centered">
            <div class="box">
                <div class="field">
                    <label class="label">{ "Username" }</label>
                    <div class="control">
                        <input
                            class="input"
                            type="text"
                            placeholder="username"
                            ref={username_ref.clone()}
                        />
                    </div>
                </div>

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

                <button {onclick} disabled={login_req.loading} class="button is-primary">
                    { "Sign in" }
                </button>

                if login_req.loading {
                    <progress class="progress is-small is-primary my-2" max="100">
                        { "10%" }
                    </progress>
                } else if *incorrect_details {
                    <div class="notification is-danger my-2">
                        <button onclick={onclick_delete} class="delete" />
                        { "Incorrect username or password" }
                    </div>
                }
            </div>
        </div>
        </>
    }
}
