pub mod account;
pub mod login;
pub mod not_found;
pub mod signup;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::contexts::is_logged_in;
use account::AccountRoute;
use login::LoginRoute;
use not_found::NotFoundRoute;
use signup::SignUpRoute;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum AppRoute {
    /// Login page
    #[at("/user/login")]
    Login,
    /// Manage account page
    #[at("/user/account")]
    Account,
    /// Create account
    #[at("/user/signup")]
    SignUp,
    /// 404 page
    #[not_found]
    #[at("/404")]
    NotFound,
}

impl AppRoute {
    pub fn switch(route: &AppRoute) -> Html {
        let not_logged_in = !is_logged_in();

        match route {
            AppRoute::Login => html! { <LoginRoute /> },
            AppRoute::SignUp => html! { <SignUpRoute /> },
            _ if not_logged_in => html! { <NotFoundRoute /> },

            AppRoute::Account => html! { <AccountRoute /> },
            AppRoute::NotFound => html! { <NotFoundRoute /> },
        }
    }
}
