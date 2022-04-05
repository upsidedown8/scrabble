//! Implementation of the [`GameStatsPage`].

use sycamore::prelude::*;

/// Props for the game stats page.
#[derive(Prop)]
pub struct Props {
    /// The id of the game to generate stats for.
    pub id_game: i32,
}

/// Page for stats on a specific game.
#[component]
pub fn GameStatsPage<G: Html>(ctx: ScopeRef, props: Props) -> View<G> {
    view! { ctx,
        p { "TODO" }
    }
}
