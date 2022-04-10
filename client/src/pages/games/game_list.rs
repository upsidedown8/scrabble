//! Implementation of the [`GaneListPage`].

use crate::{
    components::StaticErrorMsg,
    context::use_auth,
    requests::games::{list, overall_stats},
};
use api::routes::games::{GameMetadata, ListGamesResponse, OverallStatsResponse};
use sycamore::{prelude::*, suspense::Suspense};

/// Page for overall user stats and a game list.
#[component]
pub fn GameListPage<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div(class="page is-centered") {
            div {
                div(class="m-3 has-text-centered") {
                    h1(class="h1 is-size-5") { "Game list" }
                }

                Suspense {
                    fallback: view! { cx, p { "loading summary" } },
                    FetchGameSummary {}
                }

                Suspense {
                    fallback: view! { cx, p { "loading list" } },
                    FetchGameList {}
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

                let start_time = start_time.map(|date| date.to_string()).unwrap_or_default();
                let end_time = end_time.map(|date| date.to_string()).unwrap_or_default();
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
                        td { (is_over) }
                    }
                }
            })
            .collect(),
    );

    view! { cx,
        table(class="table") {
            thead {
                tr {
                    th { abbr(title="Game id") { "Game id: click for more" } }
                    th { "Start time" }
                    th { "End time" }
                    th { "Is the game over?" }
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
        hr

        h1(class="h1 is-size-5") { "Overall stats" }

        table(class="table") {
            // define the headers of the table.
            thead {
                tr {
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
                tr {
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
