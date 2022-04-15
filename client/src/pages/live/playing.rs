use crate::pages::live::app_state::{AppMsg, AppState};
use api::routes::live::ClientMsg;
use futures::channel::mpsc;
use sycamore::prelude::*;

/// Props for `Playing`.
#[derive(Prop)]
pub struct Props<'a> {
    /// A read-only signal for the current state.
    pub state: &'a ReadSignal<AppState>,
    /// Writing to this queue sends a message to the dispatch function.
    pub dispatch_write: mpsc::UnboundedSender<AppMsg>,
    /// Writing to this queue sends a message to the server.
    pub ws_write: mpsc::UnboundedSender<ClientMsg>,
}

/// Component for playing a live game.
#[component]
pub fn Playing<'a, G: Html>(cx: Scope<'a>, props: Props<'a>) -> View<G> {
    view! { cx,
        "Playing!!!!"
    }
}
