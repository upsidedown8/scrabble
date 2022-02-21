use sycamore::prelude::*;
use sycamore_router::{Route, Router, HistoryIntegration};
use contexts::auth::AuthCtx;
use components::{Navbar, Footer};
use routes::*;
use crate::contexts::auth::use_auth_ctx;

mod components;
mod contexts;
mod routes;
mod services;
mod error;

const AUTH_KEY: &str = "scrabble.auth";

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
    #[not_found]
    NotFound,
}

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
        let auth_ctx = use_auth_ctx(ctx).get();
        let auth_ctx = auth_ctx.as_ref();
        let serialized = serde_json::to_string(auth_ctx).unwrap();

        local_storage
            .set_item(AUTH_KEY, &serialized)
            .unwrap();
    });
    
    view! { ctx,
        Router {
            integration: HistoryIntegration::new(),
            view: |ctx, route: &ReadSignal<AppRoutes>| view! { ctx,
                div(class="app") {
                    Navbar {}

                    (match route.get().as_ref() {
                        AppRoutes::Home => view! { ctx, HomePage() },
                        AppRoutes::Account => view! { ctx, AccountPage() },
                        AppRoutes::Login => view! { ctx, LoginPage() },
                        AppRoutes::SignUp => view! { ctx, SignUpPage() },
                        AppRoutes::NotFound => view! { ctx, NotFoundPage() },
                    })

                    Footer {}
                }
            }
        }
    }
}
