use super::tile::Tile;
use scrabble::{
    game::tile,
    util::pos::{Pos, Premium},
};
use sycamore::prelude::*;

/// The class used to style squares with a bonus.
fn square_class(pos: Pos) -> &'static str {
    match pos.premium() {
        None => "square",
        Some(premium) => match premium {
            Premium::DoubleLetter => "square double-letter",
            Premium::DoubleWord => "square double-word",
            Premium::TripleLetter => "square triple-letter",
            Premium::TripleWord => "square triple-word",
            Premium::Start => "square start",
        },
    }
}

/// Props for `Board`.
#[derive(Prop)]
pub struct BoardProps<F> {
    /// A function of the position that was clicked.
    pub on_click: F,
    /// The Option<Tile> array for the board.
    pub cells: RcSignal<Vec<Option<tile::Tile>>>,
}

/// View the scrabble board, providing a single dimensional array containing
/// the 225 optional tiles.
#[component]
pub fn Board<F, G: Html>(cx: Scope, props: BoardProps<F>) -> View<G>
where
    F: Fn(Pos) + Clone + 'static,
{
    let cells = create_memo(cx, move || {
        Pos::iter()
            .zip(props.cells.get().as_ref())
            .map(|(p, t)| (p, *t))
            .collect::<Vec<_>>()
    });

    view! { cx,
        div(class="board") {
            Keyed {
                key: |&pos| pos,
                iterable: cells,
                view: move |cx, (pos, tile)| {
                    let on_click = props.on_click.clone();
                    let on_click = move |_| {
                        let on_click = on_click.clone();
                        on_click(pos);
                    };

                    view! { cx,
                        div(class=square_class(pos), on:click=on_click) {
                            (match tile {
                                Some(tile) => view! { cx,
                                    Tile {
                                        tile: tile,
                                    }
                                },
                                None => view! { cx, }
                            })
                        }
                    }
                },
            }
        }
    }
}
