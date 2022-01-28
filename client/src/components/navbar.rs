use yew::prelude::*;
use yew_router::prelude::*;

use crate::{contexts::use_auth_context, routes::AppRoute};

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let auth = use_auth_context();

    html! {
        <div class="navbar" role="navigation" aria-label="main navigation">
            <Link<AppRoute> classes="navbar-item" to={AppRoute::Login}>{ "Login" }</Link<AppRoute>>

            if auth.is_logged_in() {
                <Link<AppRoute> classes="navbar-item" to={AppRoute::Account}>{ "Account" }</Link<AppRoute>>
            }
        </div>
    }
}
