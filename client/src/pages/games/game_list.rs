//! Implementation of the [`GaneListPage`].

use crate::{
    components::StaticErrorMsg,
    context::use_auth,
    pages::{format_bool, format_datetime, format_f32},
    requests::games::{list, overall_stats},
};
use api::routes::games::{GameMetadata, ListGamesResponse, OverallStatsResponse};
use sycamore::{prelude::*, suspense::Suspense};

/// Page for overall user stats and a game list.
#[component]
pub fn GameListPage<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div(class="page") {
            section {
                div(class="is-centered") {
                    Suspense {
                        fallback: view! { cx, p { "loading summary" } },
                        FetchGameSummary {}
                    }
                }
            }

            section {
                div(class="is-centered") {
                    Suspense {
                        fallback: view! { cx, p { "loading list" } },
                        FetchGameList {}
                    }
                }
            }
        }
    }
}

/// Component that makes an API request to view a list of games.
#[component]
async fn FetchGameList<G: Html>(cx: Scope<'_>) -> View<G> {
    let auth = use_auth(cx);

    match list(auth).await {
        Ok(response) => view! { cx,
            ViewList(response)
        },
        Err(e) => view! { cx,
            StaticErrorMsg {
                err: e,
            }
        },
    }
}

/// Component that displays a list of games.
#[component]
fn ViewList<G: Html>(cx: Scope, list: ListGamesResponse) -> View<G> {
    let ListGamesResponse { games } = list;

    let table_body = View::new_fragment(
        games
            .iter()
            .map(|meta| {
                let GameMetadata {
                    id_game,
                    start_time,
                    end_time,
                    is_over,
                } = *meta;

                let start_time = start_time.map(format_datetime).unwrap_or_default();
                let end_time = end_time.map(format_datetime).unwrap_or_default();
                let game_link = format!("/games/{id_game}/stats");

                view! { cx,
                    tr {
                        // Link to the game page.
                        td {
                            a(href=game_link) {
                                (id_game)
                            }
                        }
                        td { (start_time) }
                        td { (end_time) }
                        td { (format_bool(is_over)) }
                    }
                }
            })
            .collect(),
    );

    view! { cx,
        h1(class="h1 is-size-5") { "Game list" }

        table(class="table") {
            thead {
                tr {
                    th { abbr(title="Game id: click for more information") { "id" } }
                    th { abbr(title="Start time") { "start" } }
                    th { abbr(title="End time") { "end" } }
                    th { abbr(title="Is the game over?") { "over?" } }
                }
            }
            tbody {
                (table_body)
            }
        }
    }
}

/// Component that makes an API request to view a game summary.
#[component]
async fn FetchGameSummary<G: Html>(cx: Scope<'_>) -> View<G> {
    let auth = use_auth(cx);

    match overall_stats(auth).await {
        Ok(response) => view! { cx,
            ViewSummary(response)
        },
        Err(e) => view! { cx,
            StaticErrorMsg {
                err: e,
            }
        },
    }
}

/// Component that displays a game summary.
#[component]
fn ViewSummary<G: Html>(cx: Scope, summary: OverallStatsResponse) -> View<G> {
    let OverallStatsResponse { row } = summary;

    view! { cx,
        h1(class="h1 is-size-5") { "Overall stats" }

        table(class="table") {
            // define the headers of the table.
            thead {
                tr {
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
                tr {
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
