use api::auth::Auth;
use sycamore::prelude::{ScopeRef, RcSignal};

#[derive(Clone, Debug, PartialEq)]
pub struct AuthCtx {
    pub username: String,
    pub auth: Auth,
}

pub fn use_auth_ctx(ctx: ScopeRef) -> &RcSignal<Option<AuthCtx>> {
    ctx.use_context::<RcSignal<Option<AuthCtx>>>()
}
