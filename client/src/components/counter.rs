//! A simple counter.

use sycamore::prelude::*;

/// Props for `Counter`.
#[derive(Prop)]
pub struct Props<'a> {
    pub min: usize,
    pub max: &'a ReadSignal<usize>,
    pub count: &'a Signal<usize>,
}

/// A counter with variable max value.
#[component]
pub fn Counter<'a, G: Html>(cx: Scope<'a>, props: Props<'a>) -> View<G> {
    create_effect(cx, move || {
        let min = props.min;
        let max = *props.max.get();
        let count = *props.count.get();

        match count.clamp(min, max) {
            new_count if new_count != count => {
                props.count.set(new_count);
            }
            _ => (),
        }
    });

    view! { cx,
        div(class="counter tags has-addons are-medium") {
            button(
                class="button tag",
                on:click=|_| *props.count.modify() -= 1,
            ) { "-" }
            span(class="tag") {
                (props.count.get())
            }
            button(
                class="button tag",
                on:click=|_| *props.count.modify() += 1,
            ) { "+" }
        }
    }
}

/// Props for `FixedCounter`.
#[derive(Prop)]
pub struct FixedProps<'a> {
    /// The minimum value.
    pub min: usize,
    /// The maximum value.
    pub max: usize,
    /// The value of the counter.
    pub count: &'a Signal<usize>,
}

/// A counter with a fixed min and max value.
#[component]
pub fn FixedCounter<'a, G: Html>(cx: Scope<'a>, props: FixedProps<'a>) -> View<G> {
    let max = create_memo(cx, move || props.max);

    view! { cx,
        Counter {
            min: props.min,
            max: max,
            count: props.count,
        }
    }
}
