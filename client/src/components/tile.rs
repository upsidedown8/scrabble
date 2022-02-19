use seed::{prelude::*, *};
use scrabble::tile::Tile;

/// One of the 27 scrabble tiles
pub fn view<Msg>(tile: &Tile) -> Node<Msg> {
    div! [
        C! [ "tile" ],
        tile.to_string(),
    ]
}
