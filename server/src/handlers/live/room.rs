use api::routes::live::{ClientMsg, ServerMsg};
use std::{ops::Deref, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use tokio_stream::{wrappers::UnboundedReceiverStream, StreamExt};
use warp::ws::Message;

use super::player::Player;

/// A message sent to a room. (ClientMsg + user id).
#[derive(Debug)]
pub struct RoomMsg {
    pub id_user: i32,
    pub msg: ClientMsg,
}
impl RoomMsg {
    /// Creates a new [`RoomMsg`].
    pub fn new(id_user: i32, msg: ClientMsg) -> RoomMsg {
        RoomMsg { id_user, msg }
    }
}

/// A thread-safe handle to a particular room.
#[derive(Clone, Debug)]
pub struct RoomHandle(Arc<Mutex<Room>>);
impl From<Room> for RoomHandle {
    fn from(room: Room) -> Self {
        Self(Arc::new(Mutex::new(room)))
    }
}
impl Deref for RoomHandle {
    type Target = Mutex<Room>;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

/// A game room, manages connections and communication between clients,
/// and maintains the game state to mediate plays.
#[derive(Debug)]
pub struct Room {
    /// Sends client messages to the room.
    sender: mpsc::UnboundedSender<RoomMsg>,
    /// The players in the game.
    players: Vec<Player>,
}
impl Room {
    /// Creates a new [`RoomHandle`] from the player count and a connection to the
    /// first player.
    pub fn new(player_tx: mpsc::UnboundedSender<Message>) -> RoomHandle {
        // create a queue that allows connected clients to send messages
        // to the room (multiple producers) and the room to receive the
        // message (single consumer).
        let (sender, receiver) = mpsc::unbounded_channel::<RoomMsg>();
        let mut receiver = UnboundedReceiverStream::new(receiver);

        let room_handle = RoomHandle::from(Self {
            sender,
            players: vec![Player::Human(Some(player_tx))],
        });

        // Spawn a task to listen for messages and act on them.
        tokio::spawn({
            let room_handle = room_handle.clone();
            async move {
                while let Some(msg) = receiver.next().await {
                    log::debug!("room message received: {msg:?}");
                    // lock the room and handle the message.
                    let mut room_lock = room_handle.lock().await;
                    room_lock.on_msg(msg).await;
                }

                log::info!("closing room");
            }
        });

        room_handle
    }
    /// Gets the sender half of the mpsc queue that can be used to send
    /// messages to the server.
    pub fn sender(&self) -> mpsc::UnboundedSender<RoomMsg> {
        self.sender.clone()
    }
    /// Adds a player to the room.
    pub fn add_player(&mut self, id_user: i32, tx: mpsc::UnboundedSender<Message>) {
        self.players.push(Player::Human(Some(tx)));
    }
    /// Disconnects a player from the room.
    pub fn disconnect_player(&mut self, id_player: i32) {
        todo!()
    }
    /// Called when a client message is received.
    pub async fn on_msg(&mut self, room_msg: RoomMsg) {
        match room_msg.msg {
            // Send chat message to all users.
            ClientMsg::Chat(chat) => {
                let server_msg = ServerMsg::Chat(room_msg.id_user, chat);
                self.send_all(&server_msg).await
            }
            msg => {
                log::error!("unexpected message: {msg:?}");
            }
        }
    }
    /// Sends a message to all connected players.
    async fn send_all(&self, msg: &ServerMsg) {
        for player in &self.players {
            player.send_msg(msg).await;
        }
    }
}
