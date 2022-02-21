use api::auth::Auth;
use sycamore::prelude::{ScopeRef, RcSignal};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthCtx {
    pub username: String,
    pub auth: Auth,
}

pub fn use_auth_ctx(ctx: ScopeRef) -> &RcSignal<Option<AuthCtx>> {
    ctx.use_context::<RcSignal<Option<AuthCtx>>>()
}
