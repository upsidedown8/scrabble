use client::App;
use log::Level;
use sycamore::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::new(Level::Trace));
    sycamore::render(|ctx| {
        view! { ctx,
            App()
        }
    });
}
