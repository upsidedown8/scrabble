use api::auth::Auth;
use sycamore::prelude::*;
use crate::contexts::auth::{use_auth_ctx, AuthCtx};

#[component]
pub fn HomePage<G: Html>(ctx: ScopeRef) -> View<G> {
    let auth_ctx = use_auth_ctx(ctx);

    let onclick = |_| auth_ctx.set(Some(AuthCtx {
        username: String::from("Ben is a bum"),
        auth: Auth(String::new()),
    }));

    view! { ctx,
        div(class="home-route") {
            h1 {
                "Home"
            }
            (format!("{:?}", auth_ctx))
            button(on:click=onclick)
        }
    }
}
