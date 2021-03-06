//! Module for displaying a leaderboard.

use crate::pages::format_f32;
use api::routes::leaderboard::LeaderboardRow as ApiLeaderboardRow;
use sycamore::prelude::*;

/// Props for the leaderboard.
#[derive(Prop)]
pub struct Props<'a> {
    /// The rows of the leaderboard.
    pub rows: &'a ReadSignal<Vec<ApiLeaderboardRow>>,
}

/// Renders a row of a leaderboard within a table.
#[component]
pub fn Leaderboard<'a, G: Html>(cx: Scope<'a>, props: Props<'a>) -> View<G> {
    view! { cx,
        div(class="columns is-centered") {
            table(class="table") {
                // define the headers of the table.
                thead {
                    tr {
                        th {abbr(title="Username") { "id" }}
                        th {abbr(title="Average score") { "score/play" }}
                        th {abbr(title="Average word length") { "wlen/play" }}
                        th {abbr(title="Average tiles placed") { "tiles/play" }}
                        th {abbr(title="Longest word length") { "longest" }}
                        th {abbr(title="Best score") { "best" }}
                        th {abbr(title="Average score per game") { "score/game" }}
                        th {abbr(title="Average score per tile") { "score/tile" }}
                        th {abbr(title="Win percentage") { "w%" }}
                    }
                }
                // define the body of the table.
                tbody {
                    Indexed {
                        iterable: props.rows,
                        view: |cx, row| view! { cx,
                            tr {
                                td { (row.username) }
                                td { (format_f32(row.avg_score_per_play)) }
                                td { (format_f32(row.avg_word_length)) }
                                td { (format_f32(row.avg_tiles_per_play)) }
                                td { (row.longest_word_length) }
                                td { (row.best_word_score) }
                                td { (format_f32(row.avg_score_per_game)) }
                                td { (format_f32(row.avg_score_per_tile)) }
                                td { (format_f32(row.win_percentage)) "%" }
                            }
                        }
                    }
                }
            }
        }
    }
}
