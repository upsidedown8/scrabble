use crate::components::board::Board;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HomePageProps {}

#[function_component(HomeRoute)]
pub fn home_route(props: &HomePageProps) -> Html {
    let tiles = [None; 225];

    html! {
        <>
            <div class="columns">
                <div class="column">
                    <h1>{ "Home page" }</h1>
                </div>
                <div class="column">
                    <Board {tiles} />
                </div>
                <div class="column">
                    <h1>{ "Home page" }</h1>
                </div>
            </div>
        </>
    }
}
