//! Provides context (data available to all components) for global
//! theming and authorization.

use api::{auth::Token, routes::users::UserDetails};
use serde::{Deserialize, Serialize};
use sycamore::prelude::{
    create_effect, create_memo, create_signal, provide_context_ref, use_context, ReadSignal, Scope,
    Signal,
};

/// Type alias for the global auth signal.
pub type AuthSignal = Signal<Option<AuthCtx>>;

/// The auth data, contains user info and the auth token.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthCtx {
    /// Username and email
    pub user_details: UserDetails,
    /// JWT from server
    pub token: Token,
}

/// HTML LocalStorage key for the auth info.
const AUTH_KEY: &str = "scrabble.auth";

/// Provides the auth context in the scope.
pub fn provide_auth_context(cx: Scope) -> &AuthSignal {
    // get a reference to the browser LocalStorage to store and
    // retrieve the authentication data.
    let local_storage = web_sys::window()
        .expect("window object")
        .local_storage()
        .expect("local storage")
        .expect("local storage enabled");

    // try to deserialize existing auth data.
    let string = local_storage.get_item(AUTH_KEY).ok().flatten();
    let deserialized = string.as_deref().map(serde_json::from_str);
    let auth_ctx: Option<AuthCtx> = deserialized.and_then(|v| v.ok());

    // provide optional auth data to the entire app
    let auth = provide_context_ref(cx, create_signal(cx, auth_ctx));

    // store new value in LocalStorage whenever the auth data is updated.
    create_effect(cx, || {
        let auth_ctx = auth.get().as_ref();
        let serialized = serde_json::to_string(auth_ctx).unwrap();

        local_storage.set_item(AUTH_KEY, &serialized).unwrap();
    });

    auth
}

/// Gets a signal containing the optional auth data.
pub fn use_auth(cx: Scope) -> &AuthSignal {
    use_context(cx)
}

/// Gets a signal that indicates whether the user is logged in.
pub fn use_logged_in(cx: Scope) -> &ReadSignal<bool> {
    let auth = use_auth(cx);

    create_memo(cx, || auth.get().is_some())
}

/// Gets a signal containing the optional user details.
pub fn use_user_details(cx: Scope) -> &ReadSignal<Option<UserDetails>> {
    let auth = use_auth(cx);

    create_memo(cx, || {
        let auth: Option<&AuthCtx> = auth.get().as_ref().as_ref();

        auth.map(|ctx| ctx.user_details)
    })
}

/// Gets a signal containing the optional auth token.
pub fn use_token<'a>(cx: Scope<'a>) -> &'a ReadSignal<Option<Token>> {
    let auth = use_auth(cx);

    create_memo(cx, || {
        let auth: Option<&AuthCtx> = auth.get().as_ref().as_ref();

        auth.map(|ctx| ctx.token)
    })
}

/// Sets the token, leaving the user_details field unchanged.
pub fn set_token(auth: &AuthSignal, token: Token) {
    if let Some(ctx) = auth.modify().as_mut() {
        *ctx = AuthCtx {
            token,
            ..ctx.clone()
        }
    }
}

/// Sets the user details, leaving the auth field unchanged.
pub fn set_user_details(auth: &AuthSignal, user_details: UserDetails) {
    if let Some(ctx) = auth.modify().as_mut() {
        *ctx = AuthCtx {
            user_details,
            ..ctx.clone()
        }
    }
}
