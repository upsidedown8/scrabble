/// A record in `tbl_tile`.
#[derive(Debug, Clone)]
pub struct Tile {
    /// Foreign key to the play in which this tile was placed.
    pub id_play: usize,
    /// The letter that was placed.
    pub letter: char,
    /// Whether the tile was blank.
    pub is_blank: bool,
    /// The position on which the tile was placed.
    pub pos: usize,
}
