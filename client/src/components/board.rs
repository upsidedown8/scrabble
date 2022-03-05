use super::square::Square;
use scrabble::{
    game::tile::Tile,
    util::pos::{Col, Pos, Row},
};
use sycamore::prelude::*;

#[derive(Prop, Clone, Copy)]
pub struct BoardRowProps<'a> {
    row: Row,
    cells: &'a [&'a Signal<Option<Tile>>],
}

#[component]
pub fn BoardRow<'a, G: Html>(ctx: ScopeRef<'a>, props: BoardRowProps<'a>) -> View<G> {
    let squares = View::new_fragment(
        Col::iter()
            .map(|col| Pos::from((props.row, col)))
            .zip(props.cells)
            .map(|(pos, &tile)| {
                view! { ctx,
                    Square {
                        pos: pos,
                        tile: tile,
                    }
                }
            })
            .collect(),
    );

    view! { ctx,
        div(class="board-row") {
            (squares)
        }
    }
}

#[derive(Prop, Clone)]
pub struct BoardProps<'a> {
    cells: Vec<&'a Signal<Option<Tile>>>,
}

/// View the scrabble board, providing a single dimensional array containing
/// the 225 optional tiles.
#[component]
pub fn Board<'a, G: Html>(ctx: ScopeRef<'a>, props: BoardProps<'a>) -> View<G> {
    let rows = View::new_fragment(
        props
            .cells
            .chunks_exact(15)
            .zip(Row::iter())
            .map(|(cells, row)| BoardRowProps { cells, row })
            .map(|row_props| {
                view! { ctx,
                    BoardRow(row_props)
                }
            })
            .collect(),
    );

    view! { ctx,
        div(class="board") {
            (rows)
        }
    }
}
