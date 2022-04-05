//! SPA web client for the Scrabble game.

// Produce a compiler warning for missing documentation.
#![warn(missing_docs)]

use crate::contexts::ScopeExt;
use components::{Footer, Navbar};
use contexts::AuthCtx;
use routes::*;
use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};

pub mod components;
pub mod contexts;
pub mod error;
pub mod routes;
pub mod services;

/// HTML LocalStorage key for the auth info.
const AUTH_KEY: &str = "scrabble.auth";

/// Represents the pages of the app.
#[derive(Route, Debug, Clone)]
pub enum AppRoutes {
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
    /// Create a live game page, requires login.
    #[to("/live/create")]
    CreateLive,
    /// Play live games page, requires login
    #[to("/live/<id_game>")]
    Live {
        /// Id of the live game.
        id_game: i32,
    },
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

/// Top level app component.
#[component]
pub fn App<G: Html>(ctx: ScopeRef) -> View<G> {
    // get a reference to the browser LocalStorage to store and
    // retrieve the authentication data.
    let local_storage = web_sys::window()
        .expect("window object")
        .local_storage()
        .expect("local storage")
        .expect("local storage enabled");

    // try to get and deserialize existing auth data.
    let string = local_storage.get_item(AUTH_KEY).ok().flatten();
    let deserialized = string.as_deref().map(serde_json::from_str);
    let auth_ctx: Option<AuthCtx> = deserialized.and_then(|v| v.ok());

    // provide optional auth data to the entire app
    let auth_ctx = ctx.create_signal(auth_ctx);
    ctx.provide_context_ref(auth_ctx);

    // store new value in LocalStorage whenever the auth data is updated.
    ctx.create_effect(move || {
        let auth_ctx = ctx.use_auth_context().get();
        let auth_ctx = auth_ctx.as_ref();
        let serialized = serde_json::to_string(auth_ctx).unwrap();

        local_storage.set_item(AUTH_KEY, &serialized).unwrap();
    });

    // stores the active state for the navbar.
    let active = ctx.create_signal(false);

    view! { ctx,
        // routes to different "pages" of the single page app
        // depending on the browser url. (Uses browser history api).
        Router {
            integration: HistoryIntegration::new(),
            view: move |ctx, route: &ReadSignal<AppRoutes>| view! { ctx,
                div(id="app") {
                    // The navbar at the top of the page.
                    Navbar(active)

                    ({let logged_in = *ctx.use_logged_in().get();
                    // when the user navigates to a page, hide the navbar on mobile.
                    active.set(false);
                    // match the route if
                    //   - the user is logged in, or
                    //   - the route doesn't require auth
                    match route.get().as_ref() {
                        // Home page.
                        AppRoutes::Home if logged_in => view! { ctx, HomePage() },

                        // User pages.
                        AppRoutes::Account if logged_in => view! { ctx, AccountPage() },
                        AppRoutes::Login if !logged_in => view! { ctx, LoginPage() },
                        AppRoutes::SignUp if !logged_in => view! { ctx, SignUpPage() },
                        AppRoutes::ResetPassword => view! { ctx, ResetPasswordPage() },
                        AppRoutes::ResetWithSecret {
                            username,
                            secret,
                        } => view! { ctx,
                            ResetWithSecretPage {
                                username: username.clone(),
                                secret: secret.clone(),
                            }
                        },

                        // Live game pages.
                        AppRoutes::CreateLive if logged_in => view! { ctx,
                            LivePage {
                                id_game: None,
                            }
                        },
                        AppRoutes::Live { id_game } if logged_in => view! { ctx,
                            LivePage {
                                id_game: Some(*id_game),
                            }
                        },

                        // Leaderboard pages.
                        AppRoutes::Leaderboard => view! { ctx, LeaderboardPage() },
                        AppRoutes::FriendsLeaderboard if logged_in => view! { ctx, FriendsLeaderboardPage() },

                        // Game stats pages.
                        AppRoutes::GameList if logged_in => view! { ctx, GameListPage() },
                        AppRoutes::GameStats { id_game } if logged_in => view! { ctx,
                            GameStatsPage {
                                id_game: *id_game,
                            }
                        },

                        // Friends pages.
                        AppRoutes::Friends => view! { ctx, FriendsPage() },

                        // Error pages.
                        AppRoutes::NotFound => view! { ctx, NotFoundPage() },
                        _ => view! { ctx, InvalidStatePage() },
                    }})

                    // The footer at the bottom of the page.
                    Footer {}
                }
            }
        }
    }
}
