/*

use crate::{routes::Response, AppState};
use rocket::{http::Status, serde::json::Json, State};

#[get("/words/<word>")]
pub fn check(state: &State<AppState>, word: String) -> Response<()> {
    match state.word_tree.contains(&word) {
        true => Response::Ok(Json::from(())),
        false => Response::Err(Status::NotFound),
    }
}


*/
