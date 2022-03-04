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

#[derive(Prop)]
pub struct SquareProps<'a> {
    pub pos: Pos,
    pub tile: &'a Signal<Option<tile::Tile>>,
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
        div(class=format!("square {}", premium_class(pos))) {
            (match *tile.get() {
                Some(tile) => view! { ctx,
                    (tile)
                },
                None => view! { ctx, }
            })
        }
    }
}
