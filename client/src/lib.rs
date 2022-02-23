//! SPA web client for the Scrabble game.

#![warn(missing_docs)]

use components::{Footer, Navbar};
use contexts::auth::AuthCtx;
use routes::*;
use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};

use crate::contexts::ScopeAuthExt;

mod components;
mod contexts;
mod error;
mod routes;
mod services;

/// HTML LocalStorage key for the auth info.
const AUTH_KEY: &str = "scrabble.auth";

/// Represents the pages of the app.
#[derive(Route, Debug, Clone, Copy)]
enum AppRoutes {
    #[to("/")]
    Home,
    #[to("/account")]
    Account,
    #[to("/login")]
    Login,
    #[to("/signup")]
    SignUp,
    #[to("/play")]
    Play,
    #[not_found]
    NotFound,
}

/// Top level app component.
#[component]
pub fn App<G: Html>(ctx: ScopeRef) -> View<G> {
    let local_storage = web_sys::window()
        .expect("window object")
        .local_storage()
        .expect("local storage")
        .expect("local storage enabled");

    let string = local_storage.get_item(AUTH_KEY).ok().flatten();
    let deserialized = string.as_deref().map(serde_json::from_str);
    let auth_ctx: Option<AuthCtx> = deserialized.and_then(|v| v.ok());

    let auth_ctx = create_rc_signal(auth_ctx);
    ctx.provide_context(auth_ctx);
    ctx.create_effect(move || {
        let auth_ctx = ctx.use_auth_context().get();
        let auth_ctx = auth_ctx.as_ref();
        let serialized = serde_json::to_string(auth_ctx).unwrap();

        local_storage.set_item(AUTH_KEY, &serialized).unwrap();
    });

    view! { ctx,
        Router {
            integration: HistoryIntegration::new(),
            view: |ctx, route: &ReadSignal<AppRoutes>| view! { ctx,
                div(id="app") {
                    Navbar {}

                    ({let logged_in = ctx.use_auth_context().get().is_some();
                    // match the route if
                    //   - the user is logged in, or
                    //   - the route doesn't require auth
                    match route.get().as_ref() {
                        AppRoutes::Login if !logged_in => view! { ctx, LoginPage() },
                        AppRoutes::SignUp if !logged_in => view! { ctx, SignUpPage() },
                        AppRoutes::Account if logged_in => view! { ctx, AccountPage() },
                        AppRoutes::Play if logged_in => view! { ctx, PlayPage() },
                        AppRoutes::Home => view! { ctx, HomePage() },
                        _ => view! { ctx, NotFoundPage() },
                    }})

                    Footer {}
                }
            }
        }
    }
}
