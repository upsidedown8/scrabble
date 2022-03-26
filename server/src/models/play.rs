/// Models a record of `tbl_play`.
#[derive(Debug)]
pub struct PlayModel {
    /// The id of the record.
    pub id_play: usize,
    /// References the record in `tbl_player` that made the play.
    pub id_player: usize,
    /// The letters that were added to the player's rack.
    pub letters_added: String,
}
