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
pub struct BoardProps<'a, F> {
    /// A function of the position that was clicked.
    pub on_click: F,
    /// The Option<Tile> array for the board.
    pub cells: &'a ReadSignal<Vec<Option<tile::Tile>>>,
}

/// View the scrabble board, providing a single dimensional array containing
/// the 225 optional tiles.
#[component]
pub fn Board<'a, F, G: Html>(cx: Scope<'a>, props: BoardProps<'a, F>) -> View<G>
where
    F: Fn(Pos) + Clone + 'a,
{
    let on_click = create_ref(cx, props.on_click);
    let squares = create_memo(cx, move || {
        let cells = props.cells.get();
        let cells = cells.as_ref();

        View::new_fragment(
            Pos::iter()
                .zip(cells)
                .map(|(p, t)| (p, *t))
                .map(|(pos, tile)| {
                    let on_click = on_click.clone();
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
                                None => view! { cx,
                                    div(class="premium") {
                                        (match pos.premium() {
                                            Some(Premium::Start) => "S",
                                            Some(Premium::DoubleLetter) => "2L",
                                            Some(Premium::TripleLetter) => "3L",
                                            Some(Premium::TripleWord) => "3W",
                                            Some(Premium::DoubleWord) => "2W",
                                            None => "",
                                        })
                                    }
                                }
                            })
                        }
                    }
                })
                .collect(),
        )
    });

    view! { cx,
        div(class="board") {
            (*squares.get())
        }
    }
}
