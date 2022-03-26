use crate::{
    auth::{Jwt, Role},
    models::{with_db, Db},
};
use api::{auth::Auth, games::GameMessage};
use futures::{SinkExt, StreamExt};
use scrabble::game::{Game, PlayerNum};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::{
    ws::{Message, WebSocket, Ws},
    Filter, Rejection, Reply,
};

/// A live game.
struct LiveGame {
    game: Game,
    players: HashMap<PlayerNum, LivePlayer>,
}

/// The details of a player that is connected to a game.
enum LivePlayer {
    Ai { id_player: usize },
    User { id_player: usize, id_user: Uuid },
}

/// Struct storing live websocket connections.
#[derive(Default)]
struct ConnectedUsers {
    authenticated: HashMap<Uuid, mpsc::UnboundedSender<Message>>,
    games: Vec<LiveGame>,
}
impl ConnectedUsers {
    /// Inserts the information for an authenticated user.
    pub fn insert_authenticated(&mut self, id_user: Uuid, tx: mpsc::UnboundedSender<Message>) {
        self.authenticated.insert(id_user, tx);
    }
    /// Disconnects a user by id.
    pub fn disconnect(&mut self, id_user: &Uuid) {
        self.authenticated.remove(id_user);
    }
}

/// A thread safe handle to the [`ConnectedUsers`].
type UsersHandle = Arc<RwLock<ConnectedUsers>>;

/// The filter for the game route.
pub fn all(db: Db) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let join_game_route = warp::path("join")
        .and(warp::ws())
        .and(with_db(db))
        .and(warp::any().map(UsersHandle::default))
        .map(|ws: Ws, db: Db, users: UsersHandle| {
            // call `on_upgrade` when the websocket connects.
            ws.on_upgrade(|socket| on_upgrade(socket, db, users));

            warp::reply()
        });

    let routes = join_game_route;

    warp::path("games").and(routes)
}

/// Called when the websocket connection succeeds.
async fn on_upgrade(socket: WebSocket, db: Db, users: UsersHandle) {
    log::info!("establishing client connection... {:#?}", socket);

    // split socket into sender and reciever.
    let (tx, mut rx) = socket.split();

    // listen to `conn_rx` for an `Authenticate` message
    if let Some(Ok(msg)) = rx.next().await {
        let bytes = msg.as_bytes();
        if let Ok(GameMessage::Authenticate(Auth(token))) = bincode::deserialize(bytes) {
            if let Ok(jwt) = Jwt::from_auth_token(&token, Role::User) {
                // put `tx` and `rx` back together.
                let socket = tx.reunite(rx).unwrap();

                on_authenticate(socket, jwt, &users).await;
            } else {
                log::error!("invalid auth token");
            }
        } else {
            log::error!("expected an `Authenticate` message");
        }
    } else {
        log::error!("no initial message recieved from websocket");
    }
}
/// Called once a user succesfully authenticates with the server.
async fn on_authenticate(socket: WebSocket, jwt: Jwt, users: &UsersHandle) {
    let &id_user = jwt.id_user();

    // split socket into sender and reciever.
    let (mut conn_tx, mut conn_rx) = socket.split();

    let (tx, rx) = mpsc::unbounded_channel::<Message>();
    // turns the reciever into a stream.
    let mut rx = UnboundedReceiverStream::new(rx);

    // spawn a task to listen to `rx`, and forward messages through `conn_tx`
    // to the connected player. Messages sent to `tx` are forwarded to the user.
    tokio::task::spawn({
        let users = Arc::clone(users);
        async move {
            while let Some(msg) = rx.next().await {
                if let Err(e) = conn_tx.send(msg).await {
                    log::error!("error sending msg: {e:?}");

                    // disconnect the socket when an error occurs.
                    on_disconnect(&id_user, &users).await;
                }
            }
        }
    });

    // add sender to list.
    users.write().await.insert_authenticated(id_user, tx);

    // listen to `conn_rx` for messages sent from the connected user.
    while let Some(result) = conn_rx.next().await {
        match result {
            Ok(msg) => {
                log::info!("message recieved from user: {id_user}");

                let bytes = msg.as_bytes();

                // attempt to deserialize the binary message
                match bincode::deserialize(bytes) {
                    Ok(msg) => on_message(&id_user, msg).await,
                    Err(e) => log::error!("deserialization error: {e:?}"),
                }
            }
            Err(e) => log::error!("websocket error: {e:?}"),
        }
    }
}
/// Called when a message is recieved.
async fn on_message(id_user: &Uuid, msg: GameMessage) {
    log::info!("websocket message (from {id_user}): {msg:?}");
}
/// Called when a user disconnects (or on a websocket error).
async fn on_disconnect(id_user: &Uuid, users: &UsersHandle) {
    log::info!("disconnecting user {id_user}");

    users.write().await.disconnect(id_user);
}
