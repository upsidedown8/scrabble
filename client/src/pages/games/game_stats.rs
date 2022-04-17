//! Implementation of the [`GameStatsPage`].

use crate::{
    components::StaticErrorMsg,
    context::use_auth,
    pages::{format_bool, format_datetime, format_f32},
    requests::games::stats,
};
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
        div(class="page") {
            section {
                div(class="is-centered") {
                    div(class="m-3 has-text-centered") {
                        h1(class="h1 is-size-5") { "Game stats (id_game=" (props.id_game) ")" }
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

    let start_time = start_time.map(format_datetime).unwrap_or_default();
    let end_time = end_time.map(format_datetime).unwrap_or_default();

    view! { cx,
        hr

        h1(class="h1 is-size-5") { "Meta" }

        table(class="table") {
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
                    td { (format_bool(is_over)) }
                    td { (format_bool(is_win)) }
                }
            }
        }

        hr

        h1(class="h1 is-size-5") { "Stats" }

        table(class="table") {
            thead {
                tr {
                    th {abbr(title="Average score") { "score/play" }}
                    th {abbr(title="Average word length") { "wlen/play" }}
                    th {abbr(title="Average words per play") { "words/play" }}
                    th {abbr(title="Average tiles per play") { "tiles/play" }}
                    th {abbr(title="Longest word length") { "longest" }}
                    th {abbr(title="Best score") { "best" }}
                    th {abbr(title="Average score per tile") { "score/tile" }}
                }
            }
            tbody {
                tr {
                    td { (format_f32(avg_score_per_play)) }
                    td { (format_f32(avg_word_length)) }
                    td { (format_f32(avg_words_per_play)) }
                    td { (format_f32(avg_tiles_per_play)) }
                    td { (longest_word_length) }
                    td { (best_word_score) }
                    td { (format_f32(avg_score_per_tile)) }
                }
            }
        }
    }
}
