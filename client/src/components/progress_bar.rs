use sycamore::prelude::*;

/// An indeterminate progress bar. Only displays is `visible` is true.
#[component]
pub fn ProgressBar<'a, G: Html>(ctx: ScopeRef<'a>, visible: &'a Signal<bool>) -> View<G> {
    view! { ctx,
        (match *visible.get() {
            false => view! { ctx, },
            true => view! { ctx,
                progress(class="progress is-small is-primary mt-3") {
                    "10%"
                }
            },
        })
    }
}
