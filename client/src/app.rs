use crate::routes::{home::HomeRoute, login::LoginRoute};
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    /// Home page
    #[at("/")]
    Home,
    /// Login page
    #[at("/login")]
    Login,
    /// Create account page
    #[at("/create-account")]
    CreateAccount,
    /// Manage account page
    #[at("/account")]
    Account,
    /// Play a live game (with id)
    #[at("/live/:id")]
    Live { id: Uuid },
    /// View a past game
    #[at("/game/:id")]
    ViewGame { id: Uuid },
    /// Play a local game
    #[at("/local")]
    Local,
    /// View performance statistics
    #[at("/stats")]
    Stats,

    /// 404 page
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { <HomeRoute /> },
        Route::Login => html! { <LoginRoute /> },
        Route::CreateAccount => todo!(),
        Route::Account => todo!(),
        Route::Live { id } => todo!(),
        Route::ViewGame { id } => todo!(),
        Route::Local => todo!(),
        Route::Stats => todo!(),
        Route::NotFound => html! { "Not found" },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}
