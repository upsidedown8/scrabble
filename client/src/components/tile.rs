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
        tile::Tile::Letter(letter) => {
            let score = props.tile.score();
            let score_x = match score {
                s if s >= 10 => "10",
                _ => "14",
            };

            view! { cx,
                svg(viewBox="0 0 20 28.6") {
                    rect(width="20", height="25", y="3.6", fill="#333", ry="3.5")
                    rect(width="20", height="25", fill="#fff", ry="3.5")
                    text {
                        tspan(x="4.5", y="17", font-family="'Roboto Mono', monospace", font-size="18") {
                            (letter)
                        }
                    }
                    text {
                        tspan(x=(score_x), y="23.5", font-family="sans-serif", font-size="6") {
                            (score)
                        }
                    }
                }
            }
        }
        tile::Tile::Blank(_) => view! { cx,
            svg(class="is-blank", viewBox="0 0 20 28.6") {
                rect(width="20", height="25", y="3.6", fill="#333", ry="3.5")
                rect(width="20", height="25", fill="#fff", ry="3.5")
            }
        },
    }
}
