use seed::{prelude::*, *};
use scrabble::{pos::{Pos, PosBonus}, tile::Tile};
use super::tile;

/// The class used to style squares with a particular bonus.
fn bonus_class(bonus: PosBonus) -> &'static str {
    match bonus {
        PosBonus::DoubleLetter => "double-letter",
        PosBonus::TripleLetter => "triple-letter",
        PosBonus::DoubleWord => "double-word",
        PosBonus::TripleWord => "triple-word",
        PosBonus::Start => "start",
    }
}

/// One of 225 squares that make up the board. Each square
/// is provided its board position in `Props`, so that it
/// can render the bonus (if any), along with the (optional)
/// tile which is placed in the square.
pub fn view<Msg>((pos, tile): (Pos, &Option<Tile>)) -> Node<Msg> {
    div! [
        C! [ "square", pos.bonus().map(bonus_class) ],
        tile.as_ref().map(|t| tile::view(t))
    ]
}
