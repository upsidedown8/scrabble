use crate::{
    auth::{Jwt, Role},
    models::Db,
};
use api::{
    auth::Auth,
    games::{ChatMessage, GameMessage},
};
use futures::{SinkExt, StreamExt};
use scrabble::game::{play::Play, Game, PlayerNum};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::{
    ws::{Message, WebSocket, Ws},
    Filter, Rejection, Reply,
};

/// A thread safe handle to a [`LiveGame`].
type LiveGameHandle = Arc<RwLock<LiveGame>>;

/// A live game.
struct LiveGame {
    game: Game,
    players: HashMap<PlayerNum, LivePlayer>,
}
impl LiveGame {
    /// Gets an iterator over the [`LivePlayer`]s.
    pub fn players(&self) -> impl Iterator<Item = &LivePlayer> + '_ {
        self.players.values()
    }
}

/// The details of a player that is connected to a game.
enum LivePlayer {
    /// A computer player.
    Ai { id_player: usize },
    /// A human player.
    User { id_player: usize, id_user: Uuid },
}

/// A thread safe handle to the [`ConnectedUsers`].
type LiveGamesHandle = Arc<RwLock<LiveGames>>;

/// Struct storing live websocket connections.
struct LiveGames {
    /// A handle to the database pool.
    db: Db,
    /// Maps (id_user) -> (web socket sender)
    authenticated: HashMap<Uuid, mpsc::UnboundedSender<Message>>,
    /// Maps (id_user) -> (id_game)
    game_lookup: HashMap<Uuid, Uuid>,
    /// Maps (id_game) -> (live game data)
    /// Arc<RwLock<...>> ensures thread safe mutability.
    games: HashMap<Uuid, LiveGameHandle>,
}
impl LiveGames {
    /// Creates a new live game state from a database pool handle.
    pub fn new(db: &Db) -> Self {
        Self {
            db: db.clone(),
            authenticated: HashMap::default(),
            game_lookup: HashMap::default(),
            games: HashMap::default(),
        }
    }
    /// Inserts the information for an authenticated user.
    pub fn connect(&mut self, id_user: Uuid, tx: mpsc::UnboundedSender<Message>) {
        self.authenticated.insert(id_user, tx);
    }
    /// Disconnects a user by id.
    pub async fn disconnect(&mut self, id_user: &Uuid) {
        self.authenticated.remove(id_user);

        todo!("update the game state for the user")
    }
    pub async fn send_msg(&self, id_user: &Uuid, msg: Message) {
        if let Some(tx) = self.authenticated.get(id_user) {
            log::info!("sending message: id_user={id_user}, msg={msg:?}");

            if let Err(e) = tx.send(msg) {
                log::error!("failed to send message: {e:?}");
            }
        }
    }
    /// Gets a handle to the current game by user id, if it exists.
    pub fn game(&self, id_user: &Uuid) -> Option<&LiveGameHandle> {
        self.games.get(id_user)
    }
}

/// The combined filter for the live game route.
pub fn all(db: &Db) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    // The state for all live games and connected users.
    let live_games = Arc::new(RwLock::new(LiveGames::new(db)));

    // A filter that produces a shared reference to `live_games`.
    let live_games_filter = warp::any().map(move || live_games.clone());

    let join_game_route = warp::path("join")
        .and(warp::ws())
        .and(live_games_filter)
        .map(|ws: Ws, live_games: LiveGamesHandle| {
            // call `on_upgrade` when the websocket connects.
            ws.on_upgrade(|socket| on_upgrade(socket, live_games));

            warp::reply()
        });

    let routes = join_game_route;

    warp::path("games").and(routes)
}

/// Called when the websocket connection succeeds.
async fn on_upgrade(socket: WebSocket, live_games: LiveGamesHandle) {
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

                on_authenticate(socket, jwt, &live_games).await;
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
async fn on_authenticate(socket: WebSocket, jwt: Jwt, live_games: &LiveGamesHandle) {
    let &id_user = jwt.id_user();

    // split socket into sender and reciever.
    let (mut conn_tx, mut conn_rx) = socket.split();

    let (tx, rx) = mpsc::unbounded_channel::<Message>();
    // turns the reciever into a stream.
    let mut rx = UnboundedReceiverStream::new(rx);

    // spawn a task to listen to `rx`, and forward messages through `conn_tx`
    // to the connected player. Messages sent to `tx` are forwarded to the user.
    tokio::task::spawn({
        let users = Arc::clone(live_games);
        async move {
            // recieve a message from another part of the program.
            while let Some(msg) = rx.next().await {
                // send the message to the user and check for an error.
                if let Err(e) = conn_tx.send(msg).await {
                    log::error!("error sending msg: {e:?}");

                    // disconnect the socket when an error occurs.
                    on_disconnect(&id_user, &users).await;
                }
            }
        }
    });

    // add sender to list of authenticated users.
    live_games.write().await.connect(id_user, tx);

    // listen to `conn_rx` for messages sent from the connected user.
    while let Some(result) = conn_rx.next().await {
        match result {
            Ok(msg) => {
                log::info!("message recieved from user: {id_user}");

                let bytes = msg.as_bytes();

                // attempt to deserialize the binary message
                match bincode::deserialize(bytes) {
                    Ok(msg) => on_message(&id_user, msg, live_games).await,
                    Err(e) => log::error!("deserialization error: {e:?}"),
                }
            }
            Err(e) => log::error!("websocket error: {e:?}"),
        }
    }
}
/// Called when a message is recieved.
async fn on_message(id_user: &Uuid, msg: GameMessage, live_games: &LiveGamesHandle) {
    log::info!("websocket message (from {id_user}): {msg:?}");

    match msg {
        // the user wants to make a play.
        GameMessage::RequestPlay(play) => on_request_play(id_user, play, live_games).await,
        // the user wants to send a message
        GameMessage::RequestChatMessage(chat_msg) => {
            on_request_chat(id_user, chat_msg, live_games).await
        }
        // no other messages should be recieved.
        _ => log::error!("unexpected message from client: {msg:?}"),
    }
}
/// Called when a user requests a play.
async fn on_request_play(id_user: &Uuid, play: Play, live_games: &LiveGamesHandle) {
    todo!()
}
/// Called when a user requests a chat message.
async fn on_request_chat(id_user: &Uuid, chat_msg: ChatMessage, live_games: &LiveGamesHandle) {
    // find the game data for the user.
    match live_games.read().await.game(id_user) {
        Some(live_game_handle) => {
            // serialize the chat message
            let bytes = bincode::serialize(&chat_msg).expect("failed to serialize chat message");
            let msg = Message::binary(bytes);

            // send the chat message to all users
            for player in live_game_handle.read().await.players() {
                if let LivePlayer::User { id_user, .. } = player {
                    // send the message to this user
                    live_games.read().await.send_msg(id_user, msg.clone()).await;
                }
            }
        }
        None => {
            log::error!("no game found for user: {id_user}");
            on_disconnect(id_user, live_games).await;
        }
    }

    todo!()
}
/// Called when a user disconnects (or on a websocket error).
async fn on_disconnect(id_user: &Uuid, live_games: &LiveGamesHandle) {
    log::info!("disconnecting user {id_user}");

    live_games.write().await.disconnect(id_user).await;
}
