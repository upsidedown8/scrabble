/// A record in `tbl_play`.
#[derive(Debug)]
pub struct Play {
    /// The id of the record.
    pub id_play: usize,
    /// References the record in `tbl_player` that made the play.
    pub id_player: usize,
    /// The letters that were added to the player's rack after
    /// the play.
    pub letters_added: String,
}
