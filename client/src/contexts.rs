//! Provides context (data available to all components) for global
//! theming and authorization.

use api::{auth::Auth, users::UserDetails};
use serde::{Deserialize, Serialize};
use sycamore::prelude::{ReadSignal, Scope, Signal};

/// Extension trait, used to add a method to the `Scope` struct
/// from sycamore, which makes getting auth data more ergonomic.
pub trait ScopeExt<'a> {
    /// Gets the optional authorization data.
    fn use_auth_context(&'a self) -> &'a AuthSignal;
    /// Signal storing a boolean value for whether the
    /// user is logged in.
    fn use_logged_in(&'a self) -> &'a ReadSignal<bool>;
}

impl<'a> ScopeExt<'a> for Scope<'a> {
    fn use_auth_context(&'a self) -> &'a AuthSignal {
        self.use_context::<AuthSignal>()
    }
    fn use_logged_in(&'a self) -> &'a ReadSignal<bool> {
        let auth_ctx = self.use_auth_context();

        self.create_memo(|| auth_ctx.get().is_some())
    }
}

/// Type alias for the global auth signal.
pub type AuthSignal = Signal<Option<AuthCtx>>;

/// The auth data, contains user info and the auth token.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthCtx {
    /// Username and email
    pub details: UserDetails,
    /// JWT from server
    pub auth: Auth,
}
