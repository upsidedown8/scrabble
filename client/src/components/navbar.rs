use crate::contexts::ScopeExt;
use sycamore::prelude::*;

/// Appears at the top of every page.
#[component]
pub fn Navbar<G: Html>(ctx: ScopeRef) -> View<G> {
    let active = ctx.create_signal(false);
    let logged_in = ctx.use_logged_in();

    let menu_class = ctx.create_memo(|| match *active.get() {
        true => "navbar-menu is-active",
        false => "navbar-menu",
    });

    view! { ctx,
        nav(class="navbar is-dark is-fixed-top") {
            NavbarBrand(active)
            div(class=menu_class) {
                NavbarStart(logged_in)
                NavbarEnd(logged_in)
            }
        }
    }
}

/// The brand and hamburger menu.
#[component]
fn NavbarBrand<'a, G: Html>(ctx: ScopeRef<'a>, active: &'a Signal<bool>) -> View<G> {
    let burger_class = ctx.create_memo(|| match *active.get() {
        true => "navbar-burger is-active",
        false => "navbar-burger",
    });
    let onclick = |_| active.set(!*active.get());

    view! { ctx,
        div(class="navbar-brand") {
            a(class=burger_class, on:click=onclick) {
                span {}
                span {}
                span {}
            }
        }
    }
}

/// The start (left part) of the menu.
#[component]
fn NavbarStart<'a, G: Html>(ctx: ScopeRef<'a>, logged_in: &'a ReadSignal<bool>) -> View<G> {
    view! { ctx,
        div(class="navbar-start") {
            (if *logged_in.get() {
                view! { ctx,
                    a(class="navbar-item is-primary", href="/play") {
                        "Play"
                    }
                }
            } else {
                view! { ctx, }
            })
        }
    }
}

/// The end (right part) of the menu.
#[component]
fn NavbarEnd<'a, G: Html>(ctx: ScopeRef<'a>, logged_in: &'a ReadSignal<bool>) -> View<G> {
    let auth_ctx = ctx.use_auth_context();

    view! { ctx,
        div(class="navbar-end") {
            div(class="navbar-item") {
                div(class="buttons") {
                    (if *logged_in.get() {
                        view! { ctx,
                            a(class="button is-light", href="/account") {
                                "Account"
                            }
                            a(class="button is-primary", on:click=|_| auth_ctx.set(None), href="/") {
                                "Log out"
                            }
                        }
                    } else {
                        view! { ctx,
                            a(class="button is-primary", href="/signup") {
                                strong {
                                    "Sign up"
                                }
                            }
                            a(class="button is-light", href="/login") {
                                "Log in"
                            }
                        }
                    })
                }
            }
        }
    }
}
