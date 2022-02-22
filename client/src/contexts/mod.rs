use sycamore::prelude::Scope;

pub mod auth;

pub trait ScopeAuthExt<'a> {
    fn use_auth_context(&'a self) -> &'a auth::AuthSignal;
}

impl<'a> ScopeAuthExt<'a> for Scope<'a> {
    fn use_auth_context(&'a self) -> &'a auth::AuthSignal {
        self.use_context::<auth::AuthSignal>()
    }
}
