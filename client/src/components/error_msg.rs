use crate::error::Error;
use sycamore::prelude::*;

/// Props for the error message.
#[derive(Prop)]
pub struct ErrorMsgProps<'a> {
    /// The optional error.
    pub err: &'a Signal<Option<Error>>,
}

/// Component to display an optional error message.
///   - if there is an error, displays a danger styled message
///   - if there is no error, displays nothing
#[component]
pub fn ErrorMsg<'a, G: Html>(cx: Scope<'a>, props: ErrorMsgProps<'a>) -> View<G> {
    let msg = create_memo(cx, || match props.err.get().as_ref() {
        Some(e) => e.to_string(),
        None => String::from(""),
    });

    view! { cx,
        (match props.err.get().is_some() {
            false => view! { cx, },
            true => view! { cx,
                article(class="message is-danger mt-3") {
                    div(class="message-header") {
                        p { "Error" }
                        button(on:click=|_| props.err.set(None), class="delete") {}
                    }
                    div(class="message-body") {
                        (msg.get())
                    }
                }
            }
        })
    }
}

/// Props for `StaticErrorMsg`.
#[derive(Prop)]
pub struct StaticErrorMsgProps {
    /// The optional error message.
    pub err: Error,
}

/// An error message that does not change.
#[component]
pub fn StaticErrorMsg<G: Html>(cx: Scope, props: StaticErrorMsgProps) -> View<G> {
    view! { cx,
        article(class="message is-danger mt-3") {
            div(class="message-header") {
                p { "Error" }
            }
            div(class="message-body") {
                (props.err)
            }
        }
    }
}
