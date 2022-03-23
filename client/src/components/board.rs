use super::square::Square;
use scrabble::{game::tile::Tile, util::pos::Pos};
use sycamore::prelude::*;

#[derive(Prop)]
pub struct BoardProps<'a> {
    cells: &'a Signal<Vec<(Pos, Option<Tile>)>>,
}

/// View the scrabble board, providing a single dimensional array containing
/// the 225 optional tiles.
#[component]
pub fn Board<'a, G: Html>(ctx: ScopeRef<'a>, props: BoardProps<'a>) -> View<G> {
    view! { ctx,
        div(class="board") {
            Indexed {
                iterable: props.cells,
                view: |ctx, (pos, tile)| view! { ctx,
                    Square {
                        pos: pos,
                        tile: tile,
                    }
                }
            }
        }
    }
}
