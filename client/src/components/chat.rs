//! Components for displaying a chat/message box.

use crate::context::use_user_details;
use sycamore::{prelude::*, rt::JsCast};
use web_sys::{Event, KeyboardEvent};

/// A displayed message, containing a sender and the
/// message content.
#[derive(Clone, PartialEq)]
pub struct Msg {
    /// Generally the username (or [server]).
    pub sender: String,
    /// The message content.
    pub content: String,
}

#[derive(Prop)]
pub struct Props<F> {
    /// A callback function to send a message.
    pub on_msg: F,
    /// The messages to display.
    pub messages: RcSignal<Vec<Msg>>,
}

/// A Chat box.
#[component]
pub fn Chat<'a, F, G: Html>(cx: Scope<'a>, props: Props<F>) -> View<G>
where
    F: Fn(String) + Clone + 'a,
{
    let user_details = use_user_details(cx).get();
    let username = (*user_details).clone().unwrap().username;
    let username = create_ref(cx, username);

    let msg = create_signal(cx, String::new());
    let messages = create_ref(cx, props.messages);

    let on_keypress = move |evt: Event| {
        let keyboard_event: KeyboardEvent = evt.unchecked_into();

        if keyboard_event.key().as_str() == "Enter" {
            let msg = (*msg.get()).clone();
            let on_msg = props.on_msg.clone();
            on_msg(msg);
        }
    };

    view! { cx,
        section(class="chat") {
            div(class="field is-horizontal") {
                div(class="field-label is-normal") {
                    label(class="label") { "Send:" }
                }
                div(class="field-body") {
                    div(class="field") {
                        p(class="control") {
                            input(
                                class="input",
                                placeholder="Press [Enter] to send...",
                                bind:value=msg,
                                on:keypress=on_keypress,
                            )
                        }
                    }
                }
            }

            div(class="is-flex chatbox") {
                Indexed {
                    iterable: messages,
                    view: move |cx, msg| view! { cx,
                        Message {
                            sender: msg.sender,
                            content: msg.content,
                            username: username,
                        }
                    }
                }
            }
        }
    }
}

/// Props for `Message`.
#[derive(Prop)]
struct MessageProps<'a> {
    pub username: &'a str,
    pub sender: String,
    pub content: String,
}

/// A chat message.
#[component]
fn Message<'a, G: Html>(cx: Scope<'a>, props: MessageProps<'a>) -> View<G> {
    let class = match props.sender.as_str() {
        "server" => "is-black",
        name if name == props.username => "is-success",
        _ => "is-info",
    };

    view! { cx,
        div(class="msg") {
            span(class=format!("tag {class}")) {
                (props.sender)
            }
            (props.content)
        }
    }
}
