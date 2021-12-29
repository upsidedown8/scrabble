use common::api::users::Auth;
use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};

pub mod users;

type Response<T> = std::result::Result<Json<T>, Status>;
