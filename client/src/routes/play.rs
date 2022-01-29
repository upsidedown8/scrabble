use crate::components::board::Board;
use yew::prelude::*;

#[function_component(PlayRoute)]
pub fn play_route() -> Html {
    let tiles = use_state(|| [None; 225]);

    html! {
        <div class="play-route">
            <Board tiles={*tiles} />
        </div>
    }
}
