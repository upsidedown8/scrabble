//! Displays the player's rack of tiles.

use super::tile::Tile;
use scrabble::game::tile;
use sycamore::prelude::*;

/// Props for `Rack`.
#[derive(Prop)]
pub struct Props<'a, F> {
    /// on:click callback.
    pub on_click: F,
    /// The tiles on the rack.
    pub tiles: &'a ReadSignal<Vec<tile::Tile>>,
}

/// The Rack component.
#[component]
pub fn Rack<'a, F, G: Html>(cx: Scope<'a>, props: Props<'a, F>) -> View<G>
where
    F: Fn(tile::Tile) + Clone + 'static,
{
    view! { cx,
        div(class="rack") {
            Indexed {
                iterable: props.tiles,
                view: move |cx, tile| {
                    let on_click = props.on_click.clone();
                    let on_click = move |_| {
                        let on_click = on_click.clone();
                        on_click(tile);
                    };

                    view! { cx,
                        div(on:click=on_click) {
                            Tile {
                                tile: tile,
                            }
                        }
                    }
                }
            }
        }
    }
}
