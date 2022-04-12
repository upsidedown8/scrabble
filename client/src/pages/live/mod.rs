//! Implementation of the [`LivePage`].

use crate::components::{Board, Rack};
use scrabble::{
    game::{rack, tile::Tile},
    util::pos::Pos,
};
use sycamore::prelude::*;

/// Props for `LivePage`.
#[derive(Prop)]
pub struct Props {
    /// Optional game id. If set, joins the game.
    pub id_game: Option<i32>,
}

/// Page for playing live games.
#[component]
pub fn LivePage<G: Html>(cx: Scope, _props: Props) -> View<G> {
    let cells = Pos::iter().map(|p| (p, None)).collect();
    let cells = create_signal(cx, cells);

    let rack_tiles: Vec<_> = rack::Rack::with_str("abcdef").tiles().collect();
    let tiles = create_signal(cx, rack_tiles);

    view! { cx,
        div(class="live") {
            Board {
                cells: cells,
            }

            Rack {
                tiles: tiles,
            }
        }
    }
}
