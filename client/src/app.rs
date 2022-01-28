use crate::{components::navbar::Navbar, contexts::AuthProvider, routes::AppRoute};

use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <AuthProvider>
                <Navbar />
                <Switch<AppRoute> render={Switch::render(AppRoute::switch)} />
            </AuthProvider>
        </BrowserRouter>
    }
}
