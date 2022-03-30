//! Implementation of the [`HomePage`].

use crate::components::{Board, Tile};
use scrabble::{game::tile, util::pos::Pos};
use sycamore::prelude::*;

/// The landing page.
#[component]
pub fn HomePage<G: Html>(ctx: ScopeRef) -> View<G> {
    let cells = ctx.create_signal(
        (0..225)
            .map(|p| (Pos::from(p), Some(tile::Tile::from('a'))))
            .collect::<Vec<_>>(),
    );

    view! { ctx,
        div(class="home-route") {
            Tile(tile::Tile::from('z'))
            
            Board {
                cells: cells,
            }
        }
    }
}
