pub mod account;
pub mod login;
pub mod not_found;

use yew::prelude::*;
use yew_router::prelude::*;

use account::AccountRoute;
use login::LoginRoute;
use not_found::NotFoundRoute;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum AppRoute {
    /// Login page
    #[at("/user/login")]
    Login,
    /// Manage account page
    #[at("/user/account")]
    Account,
    /// 404 page
    #[not_found]
    #[at("/404")]
    NotFound,
}

impl AppRoute {
    pub fn switch(route: &AppRoute) -> Html {
        match route {
            AppRoute::Login => html! { <LoginRoute /> },
            AppRoute::Account => html! { <AccountRoute /> },
            AppRoute::NotFound => html! { <NotFoundRoute /> },
        }
    }
}
