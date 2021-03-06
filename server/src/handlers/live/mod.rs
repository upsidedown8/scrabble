use self::{
    game::{GameHandle, GameMsg},
    games::GamesHandle,
};
use crate::auth::{Jwt, Role};
use api::{
    auth::Token,
    routes::live::{AiDifficulty, ClientMsg, LiveError, ServerMsg},
};
use futures::{Sink, SinkExt, StreamExt};
use std::fmt::Debug;
use tokio::sync::mpsc;
use warp::ws::{Message, WebSocket};

pub mod game;
pub mod games;

/// WSS /api/live
pub async fn connected(mut ws: WebSocket, games: GamesHandle) {
    // listen to `receiver` for an `AuthMsg`.
    if let Some(Ok(msg)) = ws.next().await {
        // deserialize the message.
        let bytes = msg.as_bytes();

        if let Ok(ClientMsg::Auth(Token(token))) = bincode::deserialize(bytes) {
            if let Ok(jwt) = Jwt::from_auth_token(&token, Role::User) {
                log::info!("authenticated: {}", jwt.id_user());

                authenticated(ws, jwt, games).await;
            } else {
                send_msg(&mut ws, &ServerMsg::Error(LiveError::InvalidToken)).await;

                log::error!("invalid token: {token}");
            }
        } else {
            log::error!("failed to deserialize as auth message");
        }
    } else {
        log::error!("auth message not received");
    }

    log::info!("disconnecting user");
}

/// Called when a user has authenticated.
async fn authenticated(mut ws: WebSocket, jwt: Jwt, games: GamesHandle) {
    let id_user = jwt.id_user();

    if let Some(Ok(msg)) = ws.next().await {
        match bincode::deserialize(msg.as_bytes()) {
            Ok(client_msg) => {
                match client_msg {
                    ClientMsg::Join(id_game) => join_game(id_game, ws, jwt, games).await,
                    ClientMsg::Create {
                        ai_count,
                        ai_difficulty,
                        player_count,
                        friends_only,
                    } => {
                        create_game(
                            ai_count,
                            ai_difficulty,
                            player_count,
                            friends_only,
                            ws,
                            jwt,
                            games,
                        )
                        .await
                    }
                    msg => {
                        log::error!("unexpected message: {msg:?}");
                    }
                };
            }
            Err(e) => {
                log::error!("deserialize error: {e:?}");
            }
        }
    }

    log::info!("disconnecting client: id_user={id_user}");
}

/// Joins a game.
async fn join_game(id_game: i32, mut ws: WebSocket, jwt: Jwt, games: GamesHandle) {
    log::info!("user ({}) is joining game ({id_game})", jwt.id_user());

    // attempt to get the game by id.
    let games_read = games.read().await;
    let game = games_read.get(id_game);
    drop(games_read);

    match game {
        // if the game exists, call `playing`.
        Some(game_handle) => playing(ws, jwt, game_handle).await,
        None => {
            log::error!("game not found: {id_game}");
            send_msg(&mut ws, &ServerMsg::Error(LiveError::FailedToJoin)).await;
        }
    }
}

/// Creates a game.
async fn create_game(
    ai_count: usize,
    ai_difficulty: AiDifficulty,
    player_count: usize,
    friends_only: bool,
    mut ws: WebSocket,
    jwt: Jwt,
    games: GamesHandle,
) {
    let count = player_count + ai_count;

    // send an error if there are no players.
    if player_count == 0 {
        send_msg(&mut ws, &ServerMsg::Error(LiveError::ZeroPlayers)).await;
    }
    // send an error for too few or too many players.
    else if !(2..=4).contains(&count) {
        send_msg(&mut ws, &ServerMsg::Error(LiveError::IllegalPlayerCount)).await;
    }
    // otherwise create the game.
    else {
        log::info!("creating game for user ({})", jwt.id_user());

        let mut games_write = games.write().await;
        // provide the user id if the game is set to friends only.
        let id_user = match friends_only {
            true => Some(jwt.id_user()),
            false => None,
        };
        // create the game.
        let game_handle = games_write
            .insert(ai_count, ai_difficulty, player_count, id_user)
            .await;
        drop(games_write);

        if let Some(game_handle) = game_handle {
            playing(ws, jwt, game_handle).await;
        }
    }
}

/// Forwards messages from the user to the game, and from the
/// game to the user, until the user disconnects.
async fn playing(ws: WebSocket, jwt: Jwt, game: GameHandle) {
    log::info!("playing game: id_user={}", jwt.id_user());

    let (mut sender, mut receiver) = ws.split();
    let (tx, mut rx) = mpsc::unbounded_channel();
    let id_user = jwt.id_user();

    // Add the player to the game.
    let mut game = game.lock().await;
    let game_sender = game.sender();
    if !game.add_player(id_user, tx).await {
        // stop execution if adding the player failed.
        send_msg(&mut sender, &ServerMsg::Error(LiveError::FailedToJoin)).await;
        return;
    }
    drop(game);

    // Forward messages from `receiver` -> `game_sender`
    // (Messages from client to the game).
    let join_handle = tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            match msg {
                Ok(msg) => match bincode::deserialize(msg.as_bytes()) {
                    Ok(msg) => {
                        let msg = GameMsg::client_msg(id_user, msg);
                        game_sender.send(msg).unwrap()
                    }
                    Err(e) => log::error!("failed to deserialize: {e:?}"),
                },
                Err(e) => log::error!("error receiving message: {e:?}"),
            }
        }

        // Ensure the user is disconnected by this point by sending a disconnect
        // message to the game room.
        game_sender
            .send(GameMsg::client_msg(id_user, ClientMsg::Disconnect))
            .unwrap();
    });

    // Forward messages from `rx` -> `sender`
    // (Messages from game to the client)
    while let Some(msg) = rx.recv().await {
        send_msg(&mut sender, &msg).await;
    }

    // Ensure that both async tasks complete.
    if let Err(e) = join_handle.await {
        log::error!("failed to join receiver task: {e:?}");
    }

    log::info!("user disconnecting: {id_user}");
}

/// Attempts to sends a message to the client.
async fn send_msg<T>(ws: &mut T, msg: &ServerMsg)
where
    T: SinkExt<Message> + Unpin,
    <T as Sink<Message>>::Error: Debug,
{
    log::trace!("sending message: {msg:?}");

    let bytes = bincode::serialize(msg).unwrap();
    let msg = Message::binary(bytes);

    if let Err(e) = ws.send(msg).await {
        log::error!("failed to send message: {e:?}");
    }
}
