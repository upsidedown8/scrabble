//! Provides a context to store the user details, and API token.

use crate::routes::AppRoute;
use api::users::{UserDetails, UserLoginResponse};
use std::sync::RwLock;
use yew::prelude::*;
use yew_router::{
    history::{AnyHistory, History},
    hooks::use_history,
};

lazy_static::lazy_static! {
    /// Mutable, global variable to store the api token.
    static ref TOKEN: RwLock<Option<String>> = RwLock::new(None);
}

/// Sets the current token.
pub fn set_token(token: Option<String>) {
    if let Ok(mut lock) = TOKEN.write() {
        *lock = token;
    }
}

/// Gets the current token.
pub fn get_token() -> Option<String> {
    TOKEN.read().ok().as_deref().cloned().flatten()
}

#[derive(Clone, PartialEq)]
pub struct AuthContext {
    auth_state: UseStateHandle<Option<UserDetails>>,
    history: AnyHistory,
}

impl AuthContext {
    /// Checks whether the current user is logged in.
    pub fn is_logged_in(&self) -> bool {
        self.auth_state.is_some()
    }
    /// Logins in a user using the response from the
    /// corresponding api route and navigates to the
    /// [`Account`](AppRoute::Account) page.
    pub fn login(&self, response: UserLoginResponse) {
        set_token(Some(response.auth.0));
        self.auth_state.set(Some(response.user_details));
        self.history.push(AppRoute::Account);
    }
    /// Logs out a user and navigates to the [`Login`](AppRoute::Login)
    /// page.
    pub fn logout(&self) {
        set_token(None);
        self.auth_state.set(None);
        self.history.push(AppRoute::Login);
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct AuthProviderProps {
    pub children: Children,
}

/// Component that makes [`AuthContext`] available using
/// a [`ContextProvider`].
#[function_component(AuthProvider)]
pub fn auth_provider(props: &AuthProviderProps) -> Html {
    let history = use_history().unwrap();
    let auth_state = use_state(|| None);
    let auth_ctx = AuthContext {
        auth_state,
        history,
    };

    html! {
        <ContextProvider<AuthContext> context={auth_ctx}>
            {props.children.clone()}
        </ContextProvider<AuthContext>>
    }
}

/// Gets the currently logged in user details.
pub fn use_auth_context() -> AuthContext {
    use_context::<AuthContext>().unwrap()
}
