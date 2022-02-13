use crate::{contexts::use_auth_context, services::users};
use api::users::SignUp;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::{use_async, use_bool_toggle};

#[function_component(SignUpRoute)]
pub fn sign_up_route() -> Html {
    let auth_ctx = use_auth_context();
    let username_ref = use_node_ref();
    let email_ref = use_node_ref();
    let password_ref = use_node_ref();
    let existing_username = use_bool_toggle(false);

    let signup_req = {
        let username_ref = username_ref.clone();
        let email_ref = email_ref.clone();
        let password_ref = password_ref.clone();
        let existing_username = existing_username.clone();

        use_async(async move {
            let username = username_ref.cast::<HtmlInputElement>().unwrap().value();
            let email = email_ref.cast::<HtmlInputElement>().unwrap().value();
            let password = password_ref.cast::<HtmlInputElement>().unwrap().value();
            let user_create = SignUp {
                username,
                email,
                password,
            };

            let res = users::sign_up(&user_create)
                .await
                .map_err(|_| String::from("error"));

            existing_username.set(res.is_err());

            if let Ok(res) = &res {
                auth_ctx.login_signup(res.clone())
            }

            res
        })
    };

    let onclick = {
        let signup_req = signup_req.clone();

        Callback::from(move |_| {
            let signup_req = signup_req.clone();

            signup_req.run();
        })
    };
    let onclick_delete = {
        let existing_username = existing_username.clone();

        Callback::from(move |_| existing_username.toggle())
    };

    html! {
        <>
        <div class="signup-route columns is-flex is-vcentered is-centered">
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
                    <label class="label">{ "Email" }</label>
                    <div class="control">
                        <input
                            class="input"
                            type="email"
                            placeholder="john@example.com"
                            ref={email_ref.clone()}
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

                <button {onclick} disabled={signup_req.loading} class="button is-primary">{ "Sign up" }</button>

                if signup_req.loading {
                    <progress class="progress is-small is-primary" max="100">{ "15%" }</progress>
                } else if *existing_username {
                    <div class="notification is-danger my-2">
                        <button onclick={onclick_delete} class="delete" />
                        { "Username already exists" }
                    </div>
                }
            </div>
        </div>
        </>
    }
}
