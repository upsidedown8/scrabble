use scrabble::{
    util::pos::{Pos, PosBonus},
    game::tile,
};
use sycamore::prelude::*;

/// The class used to style squares with a bonus.
fn bonus_class(pos: Pos) -> &'static str {
    match pos.bonus() {
        None => "",
        Some(PosBonus::DoubleLetter) => "double-letter",
        Some(PosBonus::TripleLetter) => "triple-letter",
        Some(PosBonus::DoubleWord) => "double-word",
        Some(PosBonus::TripleWord) => "triple-word",
        Some(PosBonus::Start) => "start",
    }
}

pub struct SquareProps<'a> {
    pos: Pos,
    tile: &'a Signal<Option<tile::Tile>>,
}

/// One of 225 squares that make up the board. Each square
/// is provided its board position in `Props`, so that it
/// can render the bonus (if any), along with the (optional)
/// tile which is placed in the square.
#[component]
pub fn Square<'a, G: Html>(
    ctx: ScopeRef<'a>,
    SquareProps { pos, tile }: SquareProps<'a>,
) -> View<G> {
    view! { ctx,
        div(class=format!("square {}", bonus_class(pos))) {
            (match *tile.get() {
                Some(tile) => view! { ctx,
                    (tile)
                },
                None => view! { ctx, }
            })
        }
    }
}
