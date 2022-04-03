use api::routes::live::ServerMsg;
use scrabble::ai::Ai;
use tokio::sync::mpsc;

/// Either an AI player or a connected user.
#[derive(Debug)]
pub enum Player {
    /// The player is an AI.
    Ai(Ai),
    /// The player is a connected client. If the player is
    /// not connected, then an easy level AI makes the move.
    /// A connected player contains a sender which can be used
    /// to directly send a message from the server to the client
    /// over a websocket.
    Human(Option<mpsc::UnboundedSender<ServerMsg>>),
}
impl Player {
    /// Attempts to send a message to a connected client.
    pub async fn send_msg(&self, msg: ServerMsg) {
        if let Player::Human(Some(tx)) = &self {
            log::info!("sending message: {msg:?}");

            // Attempt to send the message.
            if let Err(e) = tx.send(msg) {
                log::error!("error sending message: {e:?}");
            }
        }
    }
    /// Disconnects the player.
    pub async fn disconnect(&mut self) {
        if matches!(self, Player::Human(_)) {
            *self = Player::Human(None);
        }
    }
}
