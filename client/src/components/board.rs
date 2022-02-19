use seed::{prelude::*, *};
use scrabble::{tile::Tile, pos::{Col, Row, Pos}, board::{COLS, CELLS}};
use super::square;

/// Display one of the 15 rows of the board.
fn view_row<Msg>((tiles, row): (&[Option<Tile>], Row)) -> Node<Msg> {
    div! [
        C! [ "board-row" ],
        Col::iter()
            .map(|col| Pos::from((row, col)))
            .zip(tiles)
            .map(square::view)
    ]
}

/// View the scrabble board, providing a single dimensional array containing
/// the 225 optional tiles.
pub fn view<Msg>(tiles: &[Option<Tile>; CELLS]) -> Node<Msg> {
    div! [
        C! [ "board" ],
        tiles
            .chunks_exact(COLS)
            .zip(Row::iter())
            .map(view_row)
    ]
}
