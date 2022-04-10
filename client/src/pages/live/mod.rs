//! Implementation of the [`LivePage`].

use sycamore::prelude::*;

/// Props for `LivePage`.
#[derive(Prop)]
pub struct Props {
    /// Optional game id. If set, joins the game.
    pub id_game: Option<i32>,
}

/// Page for playing live games.
#[component]
pub fn LivePage<G: Html>(_cx: Scope, _props: Props) -> View<G> {
    todo!()
}
