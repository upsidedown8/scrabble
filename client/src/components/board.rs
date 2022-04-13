use super::tile::Tile;
use scrabble::{
    game::tile,
    util::pos::{Pos, Premium},
};
use sycamore::prelude::*;

/// The class used to style squares with a bonus.
fn square_class(pos: Pos) -> &'static str {
    match pos.premium() {
        None => "square",
        Some(premium) => match premium {
            Premium::DoubleLetter => "square double-letter",
            Premium::DoubleWord => "square double-word",
            Premium::TripleLetter => "square triple-letter",
            Premium::TripleWord => "square triple-word",
            Premium::Start => "square start",
        },
    }
}

/// Props for `Board`.
#[derive(Prop)]
pub struct BoardProps<'a, F> {
    /// A function of the position that was clicked.
    pub on_click: F,
    /// The (Pos, Option<Tile>) array for the board.
    pub cells: &'a Signal<Vec<(Pos, Option<tile::Tile>)>>,
}

/// View the scrabble board, providing a single dimensional array containing
/// the 225 optional tiles.
#[component]
pub fn Board<'a, F, G: Html>(cx: Scope<'a>, props: BoardProps<'a, F>) -> View<G>
where
    F: Fn(Pos) + Copy + 'static,
{
    let on_click = move |pos| {
        log::info!("board position clicked: {pos:?}");
        (props.on_click)(pos);
    };

    view! { cx,
        div(class="board") {
            Keyed {
                key: |&pos| pos,
                iterable: props.cells,
                view: move |cx, (pos, tile)| {
                    view! { cx,
                        div(class=square_class(pos), on:click=move |_| on_click(pos)) {
                            (match tile {
                                Some(tile) => view! { cx,
                                    Tile {
                                        tile: tile,
                                    }
                                },
                                None => view! { cx, }
                            })
                        }
                    }
                },
            }
        }
    }
}
