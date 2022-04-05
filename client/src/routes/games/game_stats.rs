//! Implementation of the [`GameStatsPage`].

use crate::{components::ErrorMsg, contexts::ScopeExt, services::games::stats};
use api::routes::games::{GameMetadata, GameStatsResponse};
use sycamore::{futures::ScopeSpawnLocal, prelude::*};

/// Page for stats on a specific game.
#[component]
pub fn GameStatsPage<G: Html>(ctx: ScopeRef, id_game: i32) -> View<G> {
    // State signals
    let game_stats = ctx.create_signal(None);
    let err = ctx.create_signal(None);

    // Attempt to load the stats for this game.
    ctx.spawn_local(async move {
        let auth_ctx = ctx.use_auth_context();
        match stats(auth_ctx, id_game).await {
            Ok(res) => game_stats.set(Some(res)),
            Err(e) => err.set(Some(e)),
        }
    });

    view! { ctx,
        div(class="page is-centered") {
            div {
                div(class="m-3 has-text-centered") {
                    h1(class="h1 is-size-5") { "Game stats (for id_game =" (id_game) ")" }
                }

                ErrorMsg(err)

                ({match (*game_stats.get()).clone() {
                    None => view! { ctx, },
                    Some(stats) => {
                        let GameStatsResponse {
                            meta: GameMetadata {
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
                        } = stats;

                        let start_time = start_time
                            .map(|date| date.to_string())
                            .unwrap_or_default();
                        let end_time = end_time
                            .map(|date| date.to_string())
                            .unwrap_or_default();

                        view! { ctx,
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
                    },
                }})
            }
        }
    }
}
