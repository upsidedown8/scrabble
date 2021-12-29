use yew::prelude::*;
use yew_router::prelude::*;

/// scrabble.rs/game/view/20

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    /// Login page
    #[at("/login")]
    Login,
    /// Create account page
    #[at("/create")]
    CreateAccount,
    /// Manage account page
    #[at("/account")]
    Account,
    /// Play a live game (with id)
    #[at("/play/:id")]
    Play { id: usize },
    /// View a game
    #[at("/game/:id")]
    ViewGame { id: usize },

    /// 404 page
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        todo!()
    }
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Play { id } => html! { <h1>{ "Live game" }</h1> },
        Route::ViewGame { id } => html! { <h1>{ "Game" }</h1> },
        Route::NotFound => todo!(),
    }
}
