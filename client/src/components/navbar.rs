use sycamore::prelude::*;

use crate::contexts::ScopeAuthExt;

#[component]
pub fn Navbar<G: Html>(ctx: ScopeRef) -> View<G> {
    let auth_ctx = ctx.use_auth_context();
    let active = ctx.create_signal(false);
    let logged_in = ctx.create_memo(|| auth_ctx.get().is_some());

    let menu_class = ctx.create_memo(|| {
        match *active.get() {
            true => "navbar-menu is-active",
            false => "navbar-menu"
        }
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

#[component]
fn NavbarBrand<'a, G: Html>(ctx: ScopeRef<'a>, active: &'a Signal<bool>) -> View<G> {
    let burger_class = ctx.create_memo(|| {
        match *active.get() {
            true => "navbar-burger is-active",
            false => "navbar-burger"
        }
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

#[component]
fn NavbarStart<'a, G: Html>(ctx: ScopeRef<'a>, _logged_in: &'a ReadSignal<bool>) -> View<G> {
    view! { ctx,
        div(class="navbar-start") {
            a(class="navbar-item is-primary") {
                "Play"
            }
        }
    }
}

#[component]
fn NavbarEnd<'a, G: Html>(ctx: ScopeRef<'a>, logged_in: &'a ReadSignal<bool>) -> View<G> {
    let onlogout = |_| ctx.use_auth_context().set(None);

    view! { ctx,
        div(class="navbar-end") {
            div(class="navbar-item") {
                div(class="buttons") {
                    (match *logged_in.get() {
                        true => view! { ctx,
                            a(class="button is-light", href="/account") {
                                "Account"
                            }
                            a(class="button is-primary", on:click=onlogout, href="/") {
                                "Log out"
                            }
                        },
                        false => view! { ctx,
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
