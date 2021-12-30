// use common::api::users::Auth;
// use jwt::{Header, RegisteredClaims, Token};
use rocket::{http::Status, serde::json::Json};

pub mod users;
pub mod words;

type Response<T> = std::result::Result<Json<T>, Status>;
