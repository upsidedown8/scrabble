//! Navbar component.

use crate::context::{use_auth, use_logged_in};
use sycamore::prelude::*;

/// Properties for the `Navbar`.
#[derive(Prop)]
pub struct Props<'a> {
    /// Whether the navbar is expanded.
    pub is_expanded: &'a Signal<bool>,
}

/// Appears at the top of every page.
#[component]
pub fn Navbar<'a, G: Html>(cx: Scope<'a>, props: Props<'a>) -> View<G> {
    let menu_class = create_memo(cx, || match *props.is_expanded.get() {
        true => "navbar-menu is-active",
        false => "navbar-menu",
    });

    view! { cx,
        nav(class="navbar is-black is-fixed-top") {
            NavbarBrand(props.is_expanded)
            div(class=menu_class) {
                NavbarStart {}
                NavbarEnd {}
            }
        }
    }
}

/// The brand and hamburger menu.
#[component]
fn NavbarBrand<'a, G: Html>(cx: Scope<'a>, is_expanded: &'a Signal<bool>) -> View<G> {
    let burger_class = create_memo(cx, || match *is_expanded.get() {
        true => "navbar-burger is-active",
        false => "navbar-burger",
    });

    view! { cx,
        div(class="navbar-brand") {
            a(class=burger_class, on:click=|_| is_expanded.set(!*is_expanded.get())) {
                span {}
                span {}
                span {}
            }
        }
    }
}

/// The start (left part) of the menu.
#[component]
fn NavbarStart<G: Html>(cx: Scope) -> View<G> {
    let is_logged_in = use_logged_in(cx);

    view! { cx,
        div(class="navbar-start") {
            a(class="navbar-item is-primary", href="/leaderboard") {
                "Leaderboard"
            }
            (match *is_logged_in.get() {
                false => view! { cx, },
                true => view! { cx,
                    a(class="navbar-item is-primary", href="/live") {
                        "Play"
                    }
                    a(class="navbar-item is-primary", href="/games") {
                        "Game list"
                    }
                    a(class="navbar-item is-primary", href="/leaderboard/friends") {
                        "Friend Leaderboard"
                    }
                    a(class="navbar-item is-primary", href="/friends") {
                        "Friends"
                    }
                },
            })
        }
    }
}

/// The end (right part) of the menu.
#[component]
fn NavbarEnd<G: Html>(cx: Scope) -> View<G> {
    let is_logged_in = use_logged_in(cx);

    view! { cx,
        div(class="navbar-end") {
            div(class="navbar-item") {
                div(class="buttons") {
                    (match *is_logged_in.get() {
                        true => view! { cx, NavbarEndLoggedIn {} },
                        false => view! { cx, NavbarEndLoggedOut {} },
                    })
                }
            }
        }
    }
}

/// The end part of the menu, when logged in.
#[component]
fn NavbarEndLoggedIn<G: Html>(cx: Scope) -> View<G> {
    let auth = use_auth(cx);

    view! { cx,
        a(class="button is-light", href="/users/account") {
            "Account"
        }
        a(class="button is-primary", href="/", on:click=|_| auth.set(None)) {
            "Log out"
        }
    }
}

/// The end part of the menu, when logged out.
#[component]
fn NavbarEndLoggedOut<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        a(class="button is-primary", href="/users/signup") {
            strong {
                "Sign up"
            }
        }
        a(class="button is-light", href="/users/login") {
            "Log in"
        }
    }
}
