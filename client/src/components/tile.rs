//! A board / rack tile.

use scrabble::game::tile;
use sycamore::prelude::*;

/// Props for `Tile`.
#[derive(Prop)]
pub struct Props {
    /// The scrabble tile.
    pub tile: tile::Tile,
}

/// The tile component.
#[component]
pub fn Tile<G: Html>(cx: Scope, props: Props) -> View<G> {
    match props.tile {
        tile::Tile::Letter(letter) => view! { cx,
            div(class="scrabble-tile") {
                div(class="letter") {
                    (letter)
                }
                div(class="score") {
                    (props.tile.score())
                }
            }
        },
        tile::Tile::Blank(_) => view! { cx,
            div(class="scrabble-tile")
        },
    }
}
