use sycamore::prelude::*;
use client::App;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    sycamore::render(|ctx| view! { ctx,
        h1 { "App" }
        App()
    });
}
