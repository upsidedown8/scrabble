//! Implementation of the [`GameListPage`].

use crate::{
    components::ErrorMsg,
    contexts::ScopeExt,
    services::games::{list, overall_stats},
};
use api::routes::games::{GameMetadata, ListGamesResponse, OverallStatsResponse};
use sycamore::{futures::ScopeSpawnLocal, prelude::*};

/// Page for overall user stats and a game list.
#[component]
pub fn GameListPage<G: Html>(ctx: ScopeRef) -> View<G> {
    let auth_ctx = ctx.use_auth_context();

    // State signals
    let overall = ctx.create_signal(None);
    let game_list = ctx.create_signal(None);
    let err = ctx.create_signal(None);
    let game_count = ctx.create_memo(|| {
        game_list
            .get()
            .as_ref()
            .as_ref()
            .map(|list: &Vec<_>| list.len())
            .unwrap_or(0)
    });

    // Fetch the data.
    ctx.spawn_local(async {
        match overall_stats(auth_ctx).await {
            Ok(OverallStatsResponse { row }) => overall.set(Some(row)),
            Err(e) => err.set(Some(e)),
        }
    });
    ctx.spawn_local(async {
        match list(auth_ctx).await {
            Ok(ListGamesResponse { games }) => game_list.set(Some(games)),
            Err(e) => err.set(Some(e)),
        }
    });

    view! { ctx,
        div(class="page is-centered") {
            div {
                div(class="m-3 has-text-centered") {
                    h1(class="h1 is-size-5") { "Game list (" (game_count.get()) ")" }
                }

                ErrorMsg(err)

                // Overall stats
                (match (*overall.get()).clone() {
                    None => view! { ctx, },
                    Some(row) => view! { ctx,
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
                })

                // List of games.
                (match (*game_list.get()).clone() {
                    None => view! { ctx, },
                    Some(list) => view! { ctx,
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
                                (View::new_fragment(
                                    list
                                        .iter()
                                        .map(|meta| {
                                            let GameMetadata {
                                                id_game,
                                                start_time,
                                                end_time,
                                                is_over,
                                            } = *meta;

                                            let start_time = start_time
                                                .map(|date| date.to_string())
                                                .unwrap_or_default();
                                            let end_time = end_time
                                                .map(|date| date.to_string())
                                                .unwrap_or_default();

                                            view! { ctx,
                                                tr {
                                                    // Link to the game page.
                                                    td {
                                                        (id_game)
                                                    }
                                                    td { (start_time) }
                                                    td { (end_time) }
                                                    td { (is_over) }
                                                }
                                            }
                                        })
                                        .collect()
                                ))
                            }
                        }
                    }
                })
            }
        }
    }
}
