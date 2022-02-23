//! SPA web client for the Scrabble game.

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
#[derive(Route, Debug, Clone, Copy)]
pub enum AppRoutes {
    /// Home page
    #[to("/")]
    Home,
    /// Account page, requires login
    #[to("/account")]
    Account,
    /// Login page
    #[to("/login")]
    Login,
    /// Sigup page
    #[to("/signup")]
    SignUp,
    /// Play (games) page, requires login
    #[to("/play")]
    Play,
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
    let auth_ctx = create_rc_signal(auth_ctx);
    ctx.provide_context(auth_ctx);

    // store new value in LocalStorage whenever the auth data is updated.
    ctx.create_effect(move || {
        let auth_ctx = ctx.use_auth_context().get();
        let auth_ctx = auth_ctx.as_ref();
        let serialized = serde_json::to_string(auth_ctx).unwrap();

        local_storage.set_item(AUTH_KEY, &serialized).unwrap();
    });

    view! { ctx,
        // routes to different "pages" of the single page app
        // depending on the browser url. (Uses browser history api).
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
