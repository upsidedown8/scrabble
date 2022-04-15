//! SPA web client for the Scrabble game.

use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};

use crate::{
    components::{Footer, Navbar},
    context::{provide_auth_context, use_logged_in},
    pages::*,
};

/// Represents the pages of the app.
#[derive(Route)]
pub enum Routes {
    /// Home page
    #[to("/")]
    Home,
    /// Account page, requires login
    #[to("/users/account")]
    Account,
    /// Login page
    #[to("/users/login")]
    Login,
    /// Sigup page
    #[to("/users/signup")]
    SignUp,
    /// Page to send reset password email.
    #[to("/users/reset-password")]
    ResetPassword,
    /// Reset password from email link.
    #[to("/users/reset-password/<username>/<secret>")]
    ResetWithSecret {
        /// User to reset password for.
        username: String,
        /// Random secret from the server.
        secret: String,
    },
    /// Create or join a live game, requires login.
    #[to("/live")]
    Live,
    /// Leaderboard page.
    #[to("/leaderboard")]
    Leaderboard,
    /// Friends leaderboard page.
    #[to("/leaderboard/friends")]
    FriendsLeaderboard,
    /// Game list page.
    #[to("/games")]
    GameList,
    /// Stats for a game.
    #[to("/games/<id_game>/stats")]
    GameStats {
        /// Id of the game.
        id_game: i32,
    },
    /// View and manage friends.
    #[to("/friends")]
    Friends,
    /// Not found page
    #[not_found]
    NotFound,
}

/// Top level component for the app.
#[component]
pub fn App<G: Html>(cx: Scope) -> View<G> {
    // Allow all components and pages to access the auth data.
    provide_auth_context(cx);

    // Store the navbar expanded state.
    let is_expanded = create_signal(cx, false);

    view! { cx,
        Router {
            integration: HistoryIntegration::new(),
            view: move |cx, route: &ReadSignal<Routes>| {
                view! { cx,
                    div(class="app") {
                        // Navbar at the top of every page.
                        Navbar {
                            is_expanded: is_expanded,
                        }

                        ViewRoute {
                            is_expanded: is_expanded,
                            route: route,
                        }

                        // Footer at the bottom of every page.
                        Footer {}
                    }
                }
            }
        }
    }
}

/// Props for `ViewRoute`.
#[derive(Prop)]
struct ViewRouteProps<'a> {
    /// Whether the navbar is expanded.
    pub is_expanded: &'a Signal<bool>,
    /// The route to display.
    pub route: &'a ReadSignal<Routes>,
}

#[component]
fn ViewRoute<'a, G: Html>(cx: Scope<'a>, props: ViewRouteProps<'a>) -> View<G> {
    let is_logged_in = use_logged_in(cx);

    create_effect(cx, || {
        // Collapse the navbar whenever the user navigates to a
        // new route.
        props.route.track();
        props.is_expanded.set(false);
    });

    view! { cx,
        ({
            let logged_in = *is_logged_in.get_untracked();
            // let logged_in = true;

            // Display the route.
            match props.route.get().as_ref() {
                // Home page.
                Routes::Home => view! { cx, HomePage {} },

                // User pages.
                Routes::Account if logged_in => view! { cx, AccountPage {} },
                Routes::Login if !logged_in => view! { cx, LoginPage {} },
                Routes::SignUp if !logged_in => view! { cx, SignUpPage {} },
                Routes::ResetPassword if !logged_in => view! { cx, ResetPasswordPage {} },
                Routes::ResetWithSecret {
                    username,
                    secret,
                } if !logged_in => view! { cx,
                    ResetWithSecretPage {
                        username: username.clone(),
                        secret: secret.clone(),
                    }
                },

                // Live game pages.
                Routes::Live if logged_in => view! { cx, LivePage { } },

                // Leaderboard pages.
                Routes::Leaderboard => view! { cx, LeaderboardPage {} },
                Routes::FriendsLeaderboard if logged_in => view! { cx, FriendsLeaderboardPage {} },

                // Game stats pages.
                Routes::GameList if logged_in => view! { cx, GameListPage {} },
                Routes::GameStats { id_game } if logged_in => view! { cx,
                    GameStatsPage {
                        id_game: *id_game,
                    }
                },

                // Friends pages.
                Routes::Friends if logged_in => view! { cx, FriendsPage {} },

                // Error pages.
                Routes::NotFound => view! { cx, NotFoundPage {} },
                _ => view! { cx, InvalidStatePage {} },
            }
        })
    }
}
