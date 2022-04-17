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
    F: Fn(usize, tile::Tile) + Clone + 'static,
{
    view! { cx,
        div(class="rack tiles") {
            (View::new_fragment(
                props.tiles
                    .get()
                    .iter()
                    .copied()
                    .enumerate()
                    .map({
                        let on_click = props.on_click.clone();

                        move |(idx, tile)| {
                            let on_click = on_click.clone();
                            let on_click = move |_| on_click(idx, tile);

                            view! { cx,
                                div(on:click=on_click) {
                                    Tile {
                                        tile: tile,
                                    }
                                }
                            }
                        }
                    })
                    .collect()
            ))
        }
    }
}
