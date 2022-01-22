use super::tile;
use scrabble::game::{
    pos::{Pos, PosBonus},
    tile::Tile,
};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SquareProps {
    /// Position of the square on the board.
    pub pos: Pos,
    /// Tile placed on the square.
    pub tile: Option<Tile>,
}

/// One of 225 squares that make up the board. Each square
/// is provided its board position in `Props`, so that it
/// can render the bonus (if any), along with the (optional)
/// tile which is placed in the square.
#[function_component(Square)]
pub fn square(props: &SquareProps) -> Html {
    let bonus = match props.pos.bonus() {
        None => "",
        Some(bonus) => match bonus {
            PosBonus::DoubleLetter => "double-letter",
            PosBonus::TripleLetter => "triple-letter",
            PosBonus::DoubleWord => "double-word",
            PosBonus::TripleWord => "triple-word",
            PosBonus::Start => "start",
        },
    };
    let class = format!("square {}", bonus);

    html! {
        <div {class}>
            if let Some(tile) = props.tile {
                <tile::Tile {tile} />
            }
        </div>
    }
}
