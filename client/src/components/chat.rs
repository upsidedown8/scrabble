//! Components for displaying a chat/message box.

use sycamore::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Msg {
    pub sender: String,
    pub content: String,
}

#[derive(Prop)]
pub struct Props<'a> {
    pub messages: &'a Signal<Vec<Msg>>,
}

/// A Chat box.
pub fn Chat<'a, G: Html>(cx: Scope<'a>, props: Props<'a>) -> View<G> {
    view! { cx,
        div(class="chat") {
            Indexed {
                iterable: props.messages,
                view: |cx, msg| view! { cx,
                    Message((msg.sender, msg.content))
                }
            }
        }
    }
}

/// A chat message.
fn Message<G: Html>(cx: Scope, (sender, content): (String, String)) -> View<G> {
    view! { cx,
        div {
            span(class="tag is-info") {
                (sender)
            }
            (content)
        }
    }
}
