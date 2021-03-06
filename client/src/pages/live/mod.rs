//! Implementation of the [`LivePage`].

use crate::{
    components::StaticErrorMsg,
    context::{use_auth, use_token},
    pages::live::app_state::AppState,
    requests::live::{connect_and_authenticate, to_msg},
};
use api::routes::live::{ClientMsg, LiveError, ServerMsg};
use futures::{SinkExt, StreamExt};
use gloo_timers::future::TimeoutFuture;
use reqwasm::websocket::{futures::WebSocket, Message};
use sycamore::{futures::spawn_local_scoped, prelude::*, suspense::Suspense};
use tokio::sync::mpsc;

mod app_state;
mod create_or_join;
mod playing;

use create_or_join::CreateOrJoin;
use playing::Playing;
use sycamore_router::navigate;

/// Page for playing live games.
#[component]
pub fn LivePage<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Suspense {
            fallback: view! { cx, p { "Connecting" } },
            ConnectAndAuthenticate { }
        }
    }
}

/// Connects to the server and sends an Auth message.
#[component]
async fn ConnectAndAuthenticate<G: Html>(cx: Scope<'_>) -> View<G> {
    // Get the auth token.
    let token = use_token(cx);
    let token = (*token.get()).clone().unwrap();

    // connect to the server.
    match connect_and_authenticate(token).await {
        Ok(ws) => {
            log::info!("websocket connected");

            view! { cx, Live(ws) }
        }
        // Display an error message.
        Err(e) => view! { cx,
            StaticErrorMsg {
                err: e,
            }
        },
    }
}

/// Handles the live websocket connection.
#[component]
fn Live<G: Html>(cx: Scope, ws: WebSocket) -> View<G> {
    // Setup the `AppState`, a writer for the dispatch function,
    // and a writer that sends messages to the server.
    let Setup { state, ws_write } = setup(cx, ws);

    // Store a value to indicate whether the help message should be shown.
    let show_modal = create_ref(cx, create_rc_signal(true));

    view! { cx,
        (match state.get().as_ref() {
            AppState::Connected(connected) => view! { cx,
                CreateOrJoin {
                    ws_write: ws_write.clone(),
                    msg: connected.toast.clone(),
                    show_modal: show_modal.clone(),
                }
            },
            AppState::Playing(..) => view! { cx,
                Playing {
                    state: state,
                    ws_write: ws_write.clone(),
                }
            }
        })
    }
}

/// Returned from `setup`.
struct Setup<'a> {
    /// A read-only `AppState` signal.
    pub state: &'a ReadSignal<AppState>,
    /// Messages sent to this queue are forwarded to the
    /// server.
    pub ws_write: &'a mpsc::UnboundedSender<ClientMsg>,
}

/// Sets up queues that buffers messages sent to the dispatch function
/// and to/from the server.
fn setup(cx: Scope, ws: WebSocket) -> Setup {
    // Create a state signal and a function that takes an
    // `AppMsg` to incrementally update the state (dispatch).
    let (state, dispatch) = create_reducer(cx, AppState::default(), AppState::reduce);

    // split the websocket into a read/write pair.
    let (mut socket_write, mut socket_read) = ws.split();

    // create a queue that forwards messages sent to `ws_write` to the server.
    let (ws_write, mut ws_read) = mpsc::unbounded_channel();

    // create a queue for dispatch messages. Any messages sent to `dispatch_write`
    // will be forwarded to the dispatch function on the `AppState`.
    let (dispatch_write, mut dispatch_read) = mpsc::unbounded_channel();

    // spawn a task that reads from `socket_read` (messages from server)
    // to forward messages to the dispatch queue (writes to `dispatch_write`).
    spawn_local_scoped(cx, async move {
        let auth = use_auth(cx);

        // read from `socket_read`.
        while let Some(msg) = socket_read.next().await {
            match msg {
                // If a message is received, parse it as a `ServerMsg`.
                Ok(Message::Bytes(bytes)) => {
                    match bincode::deserialize::<ServerMsg>(&bytes) {
                        // Forward the message to the dispatch queue.
                        Ok(msg) => {
                            log::info!("message recieved: {msg:?}");

                            match msg {
                                // If the message content states that the user's token
                                // has expired, log the user out.
                                ServerMsg::Error(LiveError::InvalidToken) => {
                                    auth.set(None);
                                    navigate("/live");
                                }
                                msg => dispatch_write.send(msg).unwrap(),
                            }
                        }
                        Err(e) => log::error!("failed to deserialize: {e:?}"),
                    }
                }
                // Only binary messages should be received.
                Ok(Message::Text(txt)) => {
                    log::error!("text message received: {txt:?}");
                }
                Err(e) => {
                    log::error!("websocket error: {e:?}");
                    break;
                }
            }
        }

        // Wait 5 seconds, then reload the page.
        TimeoutFuture::new(5000).await;
        navigate("/live");
    });

    // spawn a task that reads from `ws_read` (messages that need to be sent
    // to the server) and writes to `socket_write`.
    spawn_local_scoped(cx, async move {
        // read from `ws_read`.
        while let Some(msg) = ws_read.recv().await {
            log::info!("sending message: {msg:?}");

            if let Err(e) = socket_write.send(to_msg(&msg)).await {
                log::error!("failed to send message: {e:?}");
            }
        }
    });

    // spawn a task that reads from `dispatch_read` and calls the dispatch
    // function with each received message.
    spawn_local_scoped(cx, async move {
        // read from `dispatch_read`.
        while let Some(msg) = dispatch_read.recv().await {
            // call the dispatch function with each message.
            dispatch(msg);
        }
    });

    Setup {
        state,
        ws_write: create_ref(cx, ws_write),
    }
}
