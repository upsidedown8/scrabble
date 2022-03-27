/// A record in `tbl_friend_request`.
#[derive(Debug, Clone)]
pub struct Player {
    /// Autoincrementing player id.
    pub id_player: usize,
    /// Id of the game the player is participating in,
    pub id_game: usize,
    /// Id of the user. (If `None` then the player is an ai).
    pub id_user: Option<String>,
    /// Difficulty setting of the ai (easy | medium | hard). Only set
    /// if `id_user` is not set.
    pub ai_difficulty: Option<String>,
    /// The initial letters on the player's rack.
    pub initial_rack: String,
}
