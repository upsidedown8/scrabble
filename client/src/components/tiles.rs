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
    /// The optional selected tile.
    pub selected: &'a ReadSignal<Option<(usize, tile::Tile)>>,
}

/// The Rack component.
#[component]
pub fn Tiles<'a, F, G: Html>(cx: Scope<'a>, props: Props<'a, F>) -> View<G>
where
    F: Fn(usize, tile::Tile) + Clone + 'a,
{
    let on_click = create_ref(cx, props.on_click);

    let selected_idx = create_memo(cx, || {
        let selected = *props.selected.get();
        selected.map(|(idx, _)| idx)
    });
    let tiles = create_memo(cx, move || {
        View::new_fragment(
            props
                .tiles
                .get()
                .iter()
                .copied()
                .enumerate()
                .map({
                    let on_click = on_click.clone();

                    move |(idx, tile)| {
                        let on_click = on_click.clone();
                        let on_click = move |_| on_click(idx, tile);
                        let selected_class = create_memo(cx, move || {
                            let selected_idx = *selected_idx.get();
                            match selected_idx == Some(idx) {
                                true => "is-selected",
                                false => "",
                            }
                        });

                        view! { cx,
                            div(on:click=on_click, class=(selected_class.get())) {
                                Tile {
                                    tile: tile,
                                }
                            }
                        }
                    }
                })
                .collect(),
        )
    });

    view! { cx,
        div(class="tiles") {
            (*tiles.get())
        }
    }
}
