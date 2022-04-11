//! SPA web client.

// Produce a compiler warning for missing documentation.
#![warn(missing_docs)]

use log::Level;
use sycamore::prelude::*;

mod app;
mod components;
mod context;
mod error;
mod pages;
mod requests;

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::new(Level::Trace));
    sycamore::render(|cx| {
        view! { cx,
            app::App {}
        }
    });
}
