use scrabble::game::tile;
use sycamore::prelude::*;

/// The tile component.
#[component]
pub fn Tile<G: Html>(ctx: ScopeRef, tile: tile::Tile) -> View<G> {
    let letter = tile.letter().expect("a letter");
    let score = tile.score();

    let tile_class = match tile.is_blank() {
        true => "tile is-blank",
        false => "tile",
    };

    view! { ctx,
        div(class=tile_class) {
            div(class="letter") {
                (letter)
            }
            // only view the score if it is non-zero
            (match score > 0 {
                false => view! { ctx, },
                true => view! { ctx,
                    div(class="score") {
                        (score)
                    }
                },
            })
        }
    }
}
