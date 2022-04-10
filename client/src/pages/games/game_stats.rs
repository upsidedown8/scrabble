//! Implementation of the [`GameStatsPage`].

use crate::{components::StaticErrorMsg, context::use_auth, requests::games::stats};
use api::routes::games::{GameMetadata, GameStatsResponse};
use sycamore::{prelude::*, suspense::Suspense};

/// Props for `GameStatsPage`.
#[derive(Prop)]
pub struct Props {
    /// The game id.
    pub id_game: i32,
}

/// Page for stats on a specific game.
#[component]
pub fn GameStatsPage<G: Html>(cx: Scope, props: Props) -> View<G> {
    view! { cx,
        div(class="page is-centered") {
            div {
                div(class="m-3 has-text-centered") {
                    h1(class="h1 is-size-5") { "Game stats (for id_game =" (props.id_game) ")" }
                }

                Suspense {
                    fallback: view! { cx, p { "loading" } },
                    FetchGameStats {
                        id_game: props.id_game,
                    }
                }
            }
        }
    }
}

/// Component that makes an API request to view game stats.
#[component]
async fn FetchGameStats<G: Html>(cx: Scope<'_>, props: Props) -> View<G> {
    let auth = use_auth(cx);

    match stats(auth, props.id_game).await {
        Ok(response) => view! { cx,
            ViewStats(response)
        },
        Err(e) => view! { cx,
            StaticErrorMsg {
                err: e,
            }
        },
    }
}

/// Component that displays a succesful response.
#[component]
fn ViewStats<G: Html>(cx: Scope<'_>, response: GameStatsResponse) -> View<G> {
    let GameStatsResponse {
        meta:
            GameMetadata {
                start_time,
                end_time,
                is_over,
                ..
            },
        avg_score_per_play,
        avg_word_length,
        avg_words_per_play,
        avg_tiles_per_play,
        longest_word_length,
        best_word_score,
        avg_score_per_tile,
        is_win,
    } = response;

    let start_time = start_time.map(|date| date.to_string()).unwrap_or_default();
    let end_time = end_time.map(|date| date.to_string()).unwrap_or_default();

    view! { cx,
        hr

        h1(class="h1 is-size-5") { "Meta" }

        table {
            thead {
                tr {
                    th { "Start time" }
                    th { "End time" }
                    th { "Is the game over?" }
                    th { "Did you win?" }
                }
            }
            tbody {
                tr {
                    td { (start_time) }
                    td { (end_time) }
                    td { (is_over) }
                    td { (if is_win {
                        "Yes"
                    } else {
                        "No"
                    }) }
                }
            }
        }

        hr

        h1(class="h1 is-size-5") { "Stats" }

        table(class="table") {
            thead {
                tr {
                    th {abbr(title="score/play") { "Average score" }}
                    th {abbr(title="wlen/play") { "Average word length" }}
                    th {abbr(title="words/play") { "Average words per play" }}
                    th {abbr(title="tiles/play") { "Average tiles per play" }}
                    th {abbr(title="longest") { "Longest word length" }}
                    th {abbr(title="best") { "Best score" }}
                    th {abbr(title="score/tile") { "Average score per tile" }}
                }
            }
            tbody {
                tr {
                    td { (avg_score_per_play) }
                    td { (avg_word_length) }
                    td { (avg_words_per_play) }
                    td { (avg_tiles_per_play) }
                    td { (longest_word_length) }
                    td { (best_word_score) }
                    td { (avg_score_per_tile) }
                }
            }
        }
    }
}
