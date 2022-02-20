// use sycamore::prelude::*;
// use scrabble::{tile::Tile, pos::{Col, Row, Pos}, board::{COLS, CELLS}};
// use super::square::Square;

// /// Display one of the 15 rows of the board.
// #[component(BoardRow<G>)]
// fn board_row<'a>((tiles, row): (&'a [Option<Tile>], Row)) -> View<G> {
//     // let props = KeyedProps {
//     //     iterable: Col::iter()
//     //         .map(|col| Pos::from((row, col)))
//     //         .zip(tiles)
//     //         .collect(),
//     //     // template: |props| view! {
//     //     //     Square(props)
//     //     // },
//     //     // key: |(pos, _)| pos,
//     // };

//     // view! {
//     //     div(class="board-row") {
//     //         Keyed(props)
//     //     }
//     // }

//     todo!()
// }

// pub struct BoardRowProps<'a> {
//     cells: Vec<&'a Signal<Option<Tile>>>,
// }

// pub struct BoardProps<'a> {
//     cells: &'a Signal<Vec<BoardRowProps<'a>>>,
// }

// /// View the scrabble board, providing a single dimensional array containing
// /// the 225 optional tiles.
// #[component]
// pub fn Board<G: Html>(ctx: ScopeRef, tiles: ReadSignal<[Option<Tile>; CELLS]>) -> View<G> {
//     view! { ctx,
//         div(class="board") {
//             (tiles
//                 .get()
//                 .chunks(15)
//                 .zip(Row::iter())
//                 .map(|props| view! {
//                     BoardRow(props)
//                 })
//                 .collect())
//         }
//     }
// }
