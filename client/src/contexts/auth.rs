use api::{auth::Auth, users::UserDetails};
use sycamore::prelude::RcSignal;
use serde::{Serialize, Deserialize};

pub type AuthSignal = RcSignal<Option<AuthCtx>>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthCtx {
    pub details: UserDetails,
    pub auth: Auth,
}
