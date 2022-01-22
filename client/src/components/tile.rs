use yew::prelude::*;

/// Properties for [`Tile`].
#[derive(Properties, PartialEq)]
pub struct Props {
    /// The tile to display.
    pub tile: scrabble::game::tile::Tile,
}

/// One of the 27 scrabble tiles.
#[function_component(Tile)]
pub fn tile(props: &Props) -> Html {
    html! {
        <div class="tile">
            { props.tile }
        </div>
    }
}
