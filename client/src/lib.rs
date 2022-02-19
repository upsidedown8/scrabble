use seed::prelude::*;

mod components;
mod routes;
mod services;

/// Starts the app within the html element with id of `root_elem_id`.
pub fn start_app(root_elem_id: &str) {
    use routes::{init, update, view};
    
    App::start(root_elem_id, init, update, view);
}
