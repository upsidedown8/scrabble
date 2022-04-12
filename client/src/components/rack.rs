//! Displays the player's rack of tiles.

use super::tile::Tile;
use scrabble::game::tile;
use sycamore::prelude::*;

/// Props for `Rack`.
#[derive(Prop)]
pub struct Props<'a> {
    /// The tiles on the rack.
    pub tiles: &'a ReadSignal<Vec<tile::Tile>>,
}

/// The Rack component.
#[component]
pub fn Rack<'a, G: Html>(cx: Scope<'a>, props: Props<'a>) -> View<G> {
    view! { cx,
        div(class="rack") {
            Indexed {
                iterable: props.tiles,
                view: |cx, tile| view! { cx,
                    Tile {
                        tile: tile,
                    }
                }
            }
        }
    }
}
