use sycamore::prelude::*;

mod app;
mod components;
mod context;
mod error;
mod pages;
mod requests;

fn main() {
    sycamore::render(|cx| {
        view! { cx, app::App() }
    });
}
