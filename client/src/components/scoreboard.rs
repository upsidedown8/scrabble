use api::routes::live::Player;
use std::collections::HashMap;
use sycamore::prelude::*;

/// Props for `Scoreboard`.
#[derive(Prop)]
pub struct Props<'a> {
    /// The player scores.
    pub scores: &'a ReadSignal<HashMap<Player, usize>>,
}

/// A scoreboard table.
#[component]
pub fn Scoreboard<'a, G: Html>(cx: Scope<'a>, props: Props<'a>) -> View<G> {
    view! { cx,
        section(class="scoreboard") {
            table(class="table has-text-white has-background-black is-fullwidth") {
                thead {
                    th { "Username" }
                    th { "Score" }
                }
                tbody {
                    ({
                        // sort by score,
                        let scores = (*props.scores.get()).clone();
                        let mut scores = scores
                            .into_iter()
                            .map(|(player, score)| (player.username, score))
                            .collect::<Vec<_>>();
                        scores.sort_by_key(|(_, score)| usize::MAX - score);

                        View::new_fragment(
                            scores
                                .into_iter()
                                .map(|(username, score)| view! { cx,
                                    tr {
                                        td { (username) }
                                        td { (score) }
                                    }
                                })
                                .collect()
                        )
                    })
                }
            }
        }
    }
}
