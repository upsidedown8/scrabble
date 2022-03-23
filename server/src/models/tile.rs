#[derive(Debug)]
pub struct Tile {
    id_tile: usize,
    id_move: usize,
    letter: char,
    is_blank: bool,
    pos: usize,
}
