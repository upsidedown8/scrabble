//! A pop-up notification.

use sycamore::prelude::*;

/// Props for `Toast`.
#[derive(Prop)]
pub struct Props {
    /// The optional message to display
    pub msg: RcSignal<Option<String>>,
}

/// The Toast component.
#[component]
pub fn Toast<G: Html>(cx: Scope, props: Props) -> View<G> {
    view! { cx,
        ({
            let msg_rc = props.msg.clone();

            match (*props.msg.get()).clone() {
                None => view! { cx, },
                Some(msg) => view! { cx,
                    div(class="toast") {
                        article(class="message") {
                            div(class="message-header") {
                                p { "Message" }
                                button(class="delete", on:click=move |_| msg_rc.set(None))
                            }

                            div(class="message-body") {
                                (msg)
                            }
                        }
                    }
                }
            }
        })
    }
}
