use crate::{
    auth::{Jwt, Role},
    models::{with_db, Db},
};
use api::{
    auth::Auth,
    games::{ChatMessage, GameMessage},
};
use futures::{SinkExt, StreamExt};
use scrabble::{
    ai::Ai,
    error::GameResult,
    game::{play::Play, Game, GameStatus, PlayerNum},
    util::fsm::FastFsm,
};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::{
    sync::{mpsc, RwLock, RwLockReadGuard, RwLockWriteGuard},
    time::sleep,
};
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::{
    ws::{Message, WebSocket, Ws},
    Filter, Rejection, Reply,
};

/// The number of seconds before a player is removed for inactivity.
const TIMEOUT_SECONDS: u64 = 60;

/// A thread safe handle to a [`LiveGame`].
type LiveGameHandle = Arc<RwLock<LiveGame>>;

/// A live game.
struct LiveGame {
    game: Game,
    players: HashMap<PlayerNum, LivePlayer>,
    users: HashMap<Uuid, PlayerNum>,
    play_count: usize,
}
impl LiveGame {
    /// Gets the information for a player from their number.
    pub fn player_info(&self, player_num: PlayerNum) -> &LivePlayer {
        &self.players[&player_num]
    }
    /// Gets an iterator over the user ids of the connected players.
    pub fn players(&self) -> impl Iterator<Item = &Uuid> + '_ {
        self.users.keys()
    }
    /// Gets the player number for the user.
    pub fn player_num(&self, id_user: &Uuid) -> Option<PlayerNum> {
        self.users.get(id_user).copied()
    }
    /// Gets the player [`Uuid`] from their [`PlayerNum`].
    pub fn player_uuid(&self, player_num: PlayerNum) -> Option<Uuid> {
        self.users
            .iter()
            .find(|&(_, v)| v == &player_num)
            .map(|(k, _)| *k)
    }
    /// Borrows the [`Game`] field immutably.
    pub fn game(&self) -> &Game {
        &self.game
    }
    /// Attempts to make a board placement.
    pub fn make_play(&mut self, play: &Play, fsm: &Arc<FastFsm>, db: Db) -> GameResult<()> {
        match self.game.make_play(play, fsm.as_ref()) {
            Ok(()) => {
                self.play_count += 1;

                // todo!("insert database record");

                Ok(())
            }
            Err(e) => Err(e),
        }
    }
    /// Gets the number of plays.
    pub fn play_count(&self) -> usize {
        self.play_count
    }
    /// Replaces a player with an Ai opponent.
    pub fn replace_player(&mut self, id_user: &Uuid, db: Db) {
        if let Some(player_num) = self.users.remove(id_user) {
            if let Some(LivePlayer::User { id_player }) = self.players.remove(&player_num) {
                self.players.insert(
                    player_num,
                    LivePlayer::Ai {
                        id_player,
                        ai: Ai::easy(),
                    },
                );

                // todo!("update database record");
            }
        }
    }
}

/// The details of a player that is connected to a game.
#[derive(Debug)]
enum LivePlayer {
    /// A computer player.
    Ai { id_player: usize, ai: Ai },
    /// A human player.
    User { id_player: usize },
}

/// A thread safe handle to the [`ConnectedUsers`].
type LiveGamesHandle = Arc<RwLock<LiveGames>>;

/// Struct storing live websocket connections.
#[derive(Default)]
struct LiveGames {
    /// Maps (id_user) -> (web socket sender)
    authenticated: HashMap<Uuid, mpsc::UnboundedSender<Message>>,
    /// Maps (id_user) -> (id_game)
    game_lookup: HashMap<Uuid, Uuid>,
    /// Maps (id_game) -> (live game data)
    /// Arc<RwLock<...>> ensures thread safe mutability.
    games: HashMap<Uuid, LiveGameHandle>,
}
impl LiveGames {
    /// Inserts the information for an authenticated user.
    pub fn connect(&mut self, id_user: Uuid, tx: mpsc::UnboundedSender<Message>) {
        self.authenticated.insert(id_user, tx);
    }
    /// Disconnects a user by id.
    pub async fn disconnect(&mut self, id_user: &Uuid, db: Db) {
        self.authenticated.remove(id_user);

        // find the game that the user is part of:
        if let Some(live_game_handle) = self.game_by_user(id_user) {
            let mut live_game_write = live_game_handle.write().await;

            // replace the user with an easy ai.
            live_game_write.replace_player(id_user, db);
        }
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
    pub fn game_by_user(&self, id_user: &Uuid) -> Option<LiveGameHandle> {
        let id_game = self.game_lookup.get(id_user)?;
        self.games.get(id_game).cloned()
    }
}

/// The combined filter for the live game route.
pub fn all(
    db: &Db,
    fsm: Arc<FastFsm>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    // The state for all live games and connected users.
    let live_games = Arc::new(RwLock::new(LiveGames::default()));

    // A filter that produces a shared reference to `live_games`.
    let live_games_filter = warp::any().map(move || Arc::clone(&live_games));

    let join_game_route = warp::path("join")
        .and(warp::ws())
        .and(live_games_filter)
        .and(with_db(db))
        .and(warp::any().map(move || Arc::clone(&fsm)))
        .map(
            |ws: Ws, live_games: LiveGamesHandle, db: Db, fsm: Arc<FastFsm>| {
                // call `on_upgrade` when the websocket connects.
                ws.on_upgrade(|socket| on_upgrade(socket, db, fsm, live_games));

                warp::reply()
            },
        );

    let routes = join_game_route;

    warp::path("games").and(routes)
}

/// Called when the websocket connection succeeds.
async fn on_upgrade(socket: WebSocket, db: Db, fsm: Arc<FastFsm>, live_games: LiveGamesHandle) {
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

                on_authenticate(socket, jwt, db, fsm, &live_games).await;
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
async fn on_authenticate(
    socket: WebSocket,
    jwt: Jwt,
    db: Db,
    fsm: Arc<FastFsm>,
    live_games: &LiveGamesHandle,
) {
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
        let db = db.clone();
        async move {
            // recieve a message from another part of the program.
            while let Some(msg) = rx.next().await {
                // send the message to the user and check for an error.
                if let Err(e) = conn_tx.send(msg).await {
                    log::error!("error sending msg: {e:?}");

                    // disconnect the socket when an error occurs.
                    on_disconnect(&id_user, db.clone(), &users).await;
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
                    Ok(msg) => {
                        on_message(&id_user, msg, db.clone(), Arc::clone(&fsm), live_games).await
                    }
                    Err(e) => log::error!("deserialization error: {e:?}"),
                }
            }
            Err(e) => log::error!("websocket error: {e:?}"),
        }
    }
}
/// Called when a message is recieved.
async fn on_message(
    id_user: &Uuid,
    msg: GameMessage,
    db: Db,
    fsm: Arc<FastFsm>,
    live_games: &LiveGamesHandle,
) {
    log::info!("websocket message (from {id_user}): {msg:?}");

    match msg {
        // the user wants to make a play.
        GameMessage::RequestPlay(play) => on_request_play(id_user, play, db, fsm, live_games).await,
        // the user wants to send a message
        GameMessage::RequestChatMessage(chat_msg) => {
            on_request_chat(id_user, db, chat_msg, live_games).await
        }
        // no other messages should be recieved.
        _ => log::error!("unexpected message from client: {msg:?}"),
    }
}
/// Called when a user requests a play.
async fn on_request_play(
    id_user: &Uuid,
    play: Play,
    db: Db,
    fsm: Arc<FastFsm>,
    live_games: &LiveGamesHandle,
) {
    let live_games_read = live_games.read().await;
    let live_game = live_games_read.game_by_user(id_user);
    drop(live_games_read);

    match live_game {
        Some(live_game) => {
            let live_game_read = live_game.read().await;
            let game = live_game_read.game();

            // check whether the user is next to play.
            if live_game_read.player_num(id_user) == game.to_play() {
                // release the read reference so a write reference can
                // be aquired.
                drop(live_game_read);

                let mut live_game_write = live_game.write().await;
                let result = live_game_write.make_play(&play, &fsm, db.clone());

                // check whether the play is valid
                let live_games_read = live_games.read().await;
                match result {
                    Ok(()) => {
                        on_make_play(
                            id_user,
                            play,
                            db,
                            fsm,
                            live_game_write,
                            live_games_read,
                            live_games,
                        )
                        .await
                    }
                    Err(e) => {
                        log::error!("invalid play: {e:?}");
                        // send a response back to the player.
                        let msg = GameMessage::PlayError(e);
                        let msg = Message::binary(bincode::serialize(&msg).unwrap());
                        live_games_read.send_msg(id_user, msg).await;
                    }
                }
            } else {
                // The user is not in the game, disconnect.
                log::error!("attempted to play on someone else's turn: {id_user:?}");
                on_disconnect(id_user, db, live_games).await;
            }
        }
        None => {
            // The user is not in a game, so disconnect them.
            log::error!("user requested a play but was not in any game: {id_user:?}");
            on_disconnect(id_user, db, live_games).await;
        }
    }
}
/// Called when a play is made.
async fn on_make_play(
    id_user: &Uuid,
    play: Play,
    db: Db,
    fsm: Arc<FastFsm>,
    mut live_game_write: RwLockWriteGuard<'_, LiveGame>,
    live_games_read: RwLockReadGuard<'_, LiveGames>,
    live_games: &LiveGamesHandle,
) {
    let play_count = live_game_write.play_count();
    let mut current_user = *id_user;
    // send the play to all connected users.
    let msg = GameMessage::Play(play);
    let msg = Message::binary(bincode::serialize(&msg).unwrap());
    for id_user in live_game_write.players() {
        live_games_read.send_msg(id_user, msg.clone()).await;
    }

    // if there are ai players, make moves for them.
    while let &GameStatus::ToPlay(to_play) = live_game_write.game().status() {
        if let LivePlayer::Ai { ai, .. } = live_game_write.player_info(to_play) {
            // use the ai to generate the next play.
            let play = ai.next_play(fsm.as_ref(), live_game_write.game());

            // make the play.
            live_game_write
                .make_play(&play, &fsm, db.clone())
                .expect("ai play should be valid");

            // send it to the connected players.
            let msg = GameMessage::Play(play);
            let msg = Message::binary(bincode::serialize(&msg).unwrap());
            for id_user in live_game_write.players() {
                live_games_read.send_msg(id_user, msg.clone()).await;
            }
        } else {
            current_user = live_game_write
                .player_uuid(to_play)
                .expect("A connected user");
            break;
        }
    }

    // release the references.
    drop(live_game_write);
    drop(live_games_read);

    // set the timeout future for the next play.
    sleep(Duration::from_secs(TIMEOUT_SECONDS)).await;

    // check whether a play has been made.
    if let Some(live_game_handle) = live_games.read().await.game_by_user(id_user) {
        let live_game_read = live_game_handle.read().await;

        // If no play has been made for the timeout duration.
        if live_game_read.play_count() == play_count {
            // send a timeout message to each user.
            let live_games_read = live_games.read().await;
            let msg = GameMessage::Timeout(current_user);
            let msg = Message::binary(bincode::serialize(&msg).unwrap());
            for id_user in live_game_read.players() {
                live_games_read.send_msg(id_user, msg.clone()).await;
            }

            // and disconnect the user that timed out.
            on_disconnect(&current_user, db, live_games).await;
        }
    }
}
/// Called when a user requests a chat message.
async fn on_request_chat(
    id_user: &Uuid,
    db: Db,
    chat_msg: ChatMessage,
    live_games: &LiveGamesHandle,
) {
    // find the game data for the user.
    match live_games.read().await.game_by_user(id_user) {
        Some(live_game_handle) => {
            // serialize the chat message
            let bytes = bincode::serialize(&chat_msg).expect("failed to serialize chat message");
            let msg = Message::binary(bytes);

            // send the chat message to all users
            for id_user in live_game_handle.read().await.players() {
                // send the message to this user
                live_games.read().await.send_msg(id_user, msg.clone()).await;
            }
        }
        None => {
            log::error!("no game found for user: {id_user}");
            on_disconnect(id_user, db, live_games).await;
        }
    }
}
/// Called when a user disconnects (or on a websocket error).
async fn on_disconnect(id_user: &Uuid, db: Db, live_games: &LiveGamesHandle) {
    log::info!("disconnecting user {id_user}");

    live_games.write().await.disconnect(id_user, db).await;
}
