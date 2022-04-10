//! Implementation of the [`LivePage`].

use sycamore::prelude::*;

/// Props for `LivePage`.
#[derive(Prop)]
struct Props {
    /// Optional game id. If set, joins the game.
    pub id_game: Option<i32>,
}

/// Page for playing live games.
#[component]
pub fn LivePage<G: Html>(cx: Scope, props: Props) -> View<G> {
    todo!()
}
