use crate::{
    contexts::{is_logged_in, use_auth_context},
    routes::AppRoute,
};
use yew::prelude::*;
use yew_hooks::use_bool_toggle;
use yew_router::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let auth_ctx = use_auth_context();
    let onclick_logout = Callback::from(move |_| auth_ctx.logout());

    let is_expanded = use_bool_toggle(false);
    let onclick_burger = {
        let expanded_state = is_expanded.clone();
        Callback::from(move |_| expanded_state.toggle())
    };

    let is_active = match *is_expanded {
        true => "is-active",
        false => "",
    };

    html! {
        <nav class="navbar is-dark is-fixed-top" role="navigation" aria-label="main navigation">
            <div class="navbar-brand">
                {"scrabble"}

                <a onclick={onclick_burger} role="button" class={classes!("navbar-burger", is_active)} aria-label="menu" aria-expanded="false">
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                </a>
            </div>

            <div class={classes!("navbar-menu", is_active)}>
                <div class="navbar-start">
                    <a class="navbar-item">
                        <div class="buttons">
                            if is_logged_in() {
                                <Link<AppRoute> classes="button is-primary" to={AppRoute::Play}>
                                    { "Play" }
                                </Link<AppRoute>>
                            }
                        </div>
                    </a>
                </div>

                <div class="navbar-end">
                    <div class="navbar-item">
                        <div class="buttons">
                            if is_logged_in() {
                                <Link<AppRoute> classes="button is-primary" to={AppRoute::Account}>
                                    { "Account" }
                                </Link<AppRoute>>
                                <a class="button is-light" onclick={onclick_logout}>
                                    { "Log out" }
                                </a>
                            } else {
                                <Link<AppRoute> classes="button is-light" to={AppRoute::SignUp}>
                                    { "Sign up" }
                                </Link<AppRoute>>
                                <Link<AppRoute> classes="button is-primary" to={AppRoute::Login}>
                                    { "Log in" }
                                </Link<AppRoute>>
                            }
                        </div>
                    </div>
                </div>
            </div>
       </nav>
    }
}
