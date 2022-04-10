//! An indeterminate progress bar,

use sycamore::prelude::*;

/// Props for `ProgressBar`.
#[derive(Prop)]
struct Props<'a> {
    /// Whether the progress bar is visible.
    pub is_visible: &'a ReadSignal<bool>,
}

/// An indeterminate progress bar.
#[component]
pub fn Progress<'a, G: Html>(cx: Scope<'a>, props: Props<'a>) -> View<G> {
    view! { cx,
        (match *props.is_visible.get() {
            false => view! { cx, },
            true => view! { cx,
                progress(class="progress is-small is-primary mt-3") {
                    "10%"
                }
            },
        })
    }
}
