use sycamore::prelude::*;

#[component]
pub fn ProgressBar<'a, G: Html>(ctx: ScopeRef<'a>, visible: &'a ReadSignal<bool>) -> View<G> {
    match *visible.get() {
        false => view! { ctx, },
        true => view! { ctx,
            progress(class="progress is-small is-primary mt-3") {
                "10%"
            }
        },
    }
}
