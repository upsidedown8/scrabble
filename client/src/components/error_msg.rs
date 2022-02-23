use crate::error::Error;
use sycamore::prelude::*;

/// Component to display an optional error message.
///   - if there is an error, displays a danger styled message
///   - if there is no error, displays nothing
#[component]
pub fn ErrorMsg<'a, G: Html>(ctx: ScopeRef<'a>, err: &'a Signal<Option<Error>>) -> View<G> {
    let err_msg = ctx.create_memo(|| match err.get().as_ref() {
        Some(e) => e.to_string(),
        None => String::from(""),
    });

    view! { ctx,
        (if err.get().is_some() {
            view! { ctx,
                article(class="message is-danger mt-3") {
                    div(class="message-header") {
                        p { "Error" }
                        button(class="delete", on:click=|_| err.set(None))
                    }
                    div(class="message-body") {
                        (err_msg.get())
                    }
                }
            }
        } else {
            view! { ctx, }
        })
    }
}
