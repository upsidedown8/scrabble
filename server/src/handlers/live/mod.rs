use self::{room::RoomMsg, rooms::RoomsHandle};
use crate::{
    auth::{Jwt, Role},
    db::Db,
    fsm::FsmHandle,
    handlers::live::room::Room,
};
use api::{auth::Auth, routes::live::ClientMsg};
use futures::{stream::SplitSink, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};

pub mod player;
pub mod room;
pub mod rooms;

/// WSS /api/live
pub async fn connect(ws: WebSocket, db: Db, fsm: FsmHandle, rooms: RoomsHandle) {
    // split socket into sender and reciever.
    let (sender, mut receiver) = ws.split();

    // listen to `receiver` for an `AuthMsg`.
    if let Some(Ok(msg)) = receiver.next().await {
        // deserialize the message.
        let bytes = msg.as_bytes();

        if let Ok(ClientMsg::Auth(Auth(token))) = bincode::deserialize(bytes) {
            if let Ok(jwt) = Jwt::from_auth_token(&token, Role::User) {
                let ws = sender.reunite(receiver).unwrap();

                authenticated(ws, jwt, rooms).await;
            } else {
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
async fn authenticated(ws: WebSocket, jwt: Jwt, rooms: RoomsHandle) {
    let id_user = jwt.id_user();
    let (sender, receiver) = ws.split();

    // while let Some(Ok(msg)) = receiver.next().await {
    //     // Listen for either a `Join` or `Create` message from the user,
    //     // then add them to a game.
    //     let room_sender = match bincode::deserialize(msg.as_bytes()) {
    //         // Player wants to jon a game.
    //         Ok(ClientMsg::Join(id_room)) => {
    //             // obtain a mutable reference to the room.
    //             let rooms_handle = rooms.read().await.handle(id_room);
    //             let room_handle = rooms_handle.lock().await;

    //             // add the player to the room.
    //             room_handle.add_player(id_user, sender.clone());
    //             room_handle.sender()
    //         }
    //         // Player wants to create a game.
    //         Ok(ClientMsg::Create(count)) => {
    //             // create a game and add the first player.
    //             let id_room = Room::new_with_player(id_user, sender.clone(), count);
    //             // add the room to the list of rooms.
    //             let rooms_handle = rooms.write().await.insert(id_room);
    //             room.sender()
    //         }
    //         // No other messages are allowed at this point.
    //         Ok(msg) => {
    //             log::error!("unexpected message: {msg:?}");
    //             break;
    //         }
    //         Err(e) => {
    //             log::error!("error deserializing message: {e:?}");
    //             break;
    //         }
    //     };

    //     // forward messages to the room.
    //     forward_messages(id_user, &mut receiver, &room_sender).await;
    // }

    log::info!("disconnecting client: id_user={id_user}");
}

// /// Forwards received messages from the client to the server.
// async fn forward_messages(
//     id_user: i32,
//     from: &mut SplitSink<WebSocket, Message>,
//     to: &mpsc::UnboundedSender<RoomMsg>,
// ) {
//     while let Some(msg) = from.next().await {
//         match bincode::deserialize(msg.as_bytes()) {
//             Ok(msg) => {
//                 let room_msg = RoomMsg::new(id_user, msg);

//                 if let Err(e) = to.send(room_msg) {
//                     log::error!("failed to send message to room: {e:?}");
//                 }
//             }
//             Err(e) => {
//                 log::error!("error deserializing message: {e:?}");
//             }
//         }
//     }
// }
