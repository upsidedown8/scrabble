use super::square::Square;
use scrabble::game::pos::{Col, Pos, Row};
use yew::prelude::*;

/// Properties for [`Board`].
#[derive(Properties, PartialEq)]
pub struct BoardProps {
    /// The single dimensional array containing the
    /// tile positions.
    pub tiles: [Option<scrabble::game::tile::Tile>; 225],
}

/// The scrabble board.
#[function_component(Board)]
pub fn board(props: &BoardProps) -> Html {
    let build_row = |row| {
        Col::iter()
            .map(|col| Pos::from((row, col)))
            .map(|pos| {
                html! {
                    <Square
                        pos={pos}
                        tile={props.tiles[usize::from(pos)]}
                    />
                }
            })
            .collect::<Html>()
    };

    let rows = Row::iter()
        .map(|row| {
            html! {
                <div class="board-row">{build_row(row)}</div>
            }
        })
        .collect::<Html>();

    html! {
        <div class="board">{rows}</div>
    }
}
