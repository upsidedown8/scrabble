#[derive(Debug)]
pub struct TileModel {
    pub id_tile: usize,
    pub id_move: usize,
    pub letter: char,
    pub is_blank: bool,
    pub pos: usize,
}
