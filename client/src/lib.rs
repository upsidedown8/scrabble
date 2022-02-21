use sycamore::prelude::*;
use sycamore_router::{Route, Router, HistoryIntegration};

mod components;
mod contexts;
mod routes;
mod services;

use contexts::auth::AuthCtx;
use components::{Navbar, Footer};
use routes::*;

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
    let auth_ctx = create_rc_signal::<Option<AuthCtx>>(None);
    ctx.provide_context(auth_ctx);
    
    view! { ctx,
        Navbar {}
        Router {
            integration: HistoryIntegration::new(),
            view: |ctx, route: &ReadSignal<AppRoutes>| {
                match route.get().as_ref() {
                    AppRoutes::Home => view! { ctx, HomePage() },
                    AppRoutes::Account => view! { ctx, AccountPage() },
                    AppRoutes::Login => view! { ctx, LoginPage() },
                    AppRoutes::SignUp => view! { ctx, SignUpPage() },
                    AppRoutes::NotFound => view! { ctx, NotFoundPage() },
                }
            }
        }
        Footer {}
    }
}
