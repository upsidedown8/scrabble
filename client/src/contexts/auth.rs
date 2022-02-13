//! Provides a context to store the user details, and API token.

use crate::routes::AppRoute;
use api::users::{LoginResponse, SignUpResponse, UserDetails};
use gloo::storage::{LocalStorage, Storage};
use std::{ops::Deref, sync::RwLock};
use yew::prelude::*;
use yew_router::{
    history::{AnyHistory, History},
    hooks::use_history,
};

const TOKEN_KEY: &str = "scrabble.token";

lazy_static::lazy_static! {
    /// Mutable, global variable to store the api token.
    static ref TOKEN: RwLock<Option<String>> = match LocalStorage::get(TOKEN_KEY) {
        Ok(t) => RwLock::new(Some(t)),
        _ => RwLock::new(None),
    };
}

/// Sets the current token.
pub fn set_token(token: Option<String>) {
    match &token {
        Some(t) => LocalStorage::set(TOKEN_KEY, t).unwrap(),
        _ => LocalStorage::delete(TOKEN_KEY),
    }

    if let Ok(mut lock) = TOKEN.write() {
        *lock = token;
    }
}

/// Gets the current token.
pub fn get_token() -> Option<String> {
    TOKEN.read().ok().as_deref().cloned().flatten()
}

/// Checks whether the user is logged in
pub fn is_logged_in() -> bool {
    get_token().is_some()
}

#[derive(Clone, PartialEq)]
pub struct AuthContextHandle {
    auth_state: UseStateHandle<UserDetails>,
    history: AnyHistory,
}

impl Deref for AuthContextHandle {
    type Target = UserDetails;

    fn deref(&self) -> &Self::Target {
        &(*self.auth_state)
    }
}
impl AuthContextHandle {
    /// Logins in a user using the response from the
    /// corresponding api route and navigates to the
    /// [`Account`](AppRoute::Account) page.
    pub fn login(&self, response: LoginResponse) {
        set_token(Some(response.auth.0));
        self.auth_state.set(response.user_details);
        self.history.push(AppRoute::Account);
    }
    pub fn login_signup(&self, response: SignUpResponse) {
        let response = LoginResponse {
            auth: response.auth,
            user_details: response.user_details,
        };
        self.login(response);
    }
    /// Logs out a user and navigates to the [`Login`](AppRoute::Login)
    /// page.
    pub fn logout(&self) {
        set_token(None);
        self.auth_state.set(UserDetails::default());
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
    let auth_state = use_state(UserDetails::default);
    let auth_ctx = AuthContextHandle {
        auth_state,
        history,
    };

    html! {
        <ContextProvider<AuthContextHandle> context={auth_ctx}>
            {props.children.clone()}
        </ContextProvider<AuthContextHandle>>
    }
}

/// Gets the currently logged in user details.
pub fn use_auth_context() -> AuthContextHandle {
    use_context::<AuthContextHandle>().unwrap()
}
