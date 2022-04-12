use super::tile::Tile;
use scrabble::{
    game::tile,
    util::pos::{Pos, Premium},
};
use sycamore::prelude::*;

/// The class used to style squares with a bonus.
fn premium_class(pos: Pos) -> &'static str {
    match pos.premium() {
        None => "",
        Some(Premium::DoubleLetter) => "double-letter",
        Some(Premium::TripleLetter) => "triple-letter",
        Some(Premium::DoubleWord) => "double-word",
        Some(Premium::TripleWord) => "triple-word",
        Some(Premium::Start) => "start",
    }
}

/// Props for `Board`.
#[derive(Prop)]
pub struct BoardProps<'a> {
    /// The (Pos, Option<Tile>) array for the board.
    pub cells: &'a Signal<Vec<(Pos, Option<tile::Tile>)>>,
}

/// View the scrabble board, providing a single dimensional array containing
/// the 225 optional tiles.
#[component]
pub fn Board<'a, G: Html>(cx: Scope<'a>, props: BoardProps<'a>) -> View<G> {
    view! { cx,
        div(class="board") {
            Keyed {
                iterable: props.cells,
                view: |cx, (pos, tile)| view! { cx,
                    div(class=format!("square {}", premium_class(pos))) {
                        (match tile {
                            Some(tile) => view! { cx,
                                Tile {
                                    tile: tile,
                                }
                            },
                            None => view! { cx, }
                        })
                    }
                },
                key: |&pos| pos
            }
        }
    }
}
