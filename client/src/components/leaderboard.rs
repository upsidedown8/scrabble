//! Module for displaying a leaderboard.

use api::routes::leaderboard::LeaderboardRow as ApiLeaderboardRow;
use sycamore::prelude::*;

/// Props for the leaderboard.
#[derive(Prop)]
struct Props<'a> {
    /// The rows of the leaderboard.
    pub rows: &'a ReadSignal<Vec<ApiLeaderboardRow>>,
}

/// Renders a row of a leaderboard within a table.
#[component]
pub fn Leaderboard<'a, G: Html>(cx: Scope<'a>, props: Props<'a>) -> View<G> {
    view! { cx,
        table(class="table") {
            // define the headers of the table.
            thead {
                tr {
                    th {abbr(title="id") { "Username" }}
                    th {abbr(title="score/play") { "Average score" }}
                    th {abbr(title="wlen/play") { "Average word length" }}
                    th {abbr(title="tiles/play") { "Average tiles placed" }}
                    th {abbr(title="longest") { "Longest word length" }}
                    th {abbr(title="best") { "Best score" }}
                    th {abbr(title="score/game") { "Average score per game" }}
                    th {abbr(title="score/tile") { "Average score per tile" }}
                    th {abbr(title="w%") { "Win percentage" }}
                }
            }
            // define the body of the table.
            tbody {
                Indexed {
                    iterable: props.rows,
                    view: |cx, row| view! { cx,
                        tr {
                            td { (row.username) }
                            td { (row.avg_score_per_play) }
                            td { (row.avg_word_length) }
                            td { (row.avg_tiles_per_play) }
                            td { (row.longest_word_length) }
                            td { (row.best_word_score) }
                            td { (row.avg_score_per_game) }
                            td { (row.avg_score_per_tile) }
                            td { (row.win_percentage) }
                        }
                    }
                }
            }
        }
    }
}
