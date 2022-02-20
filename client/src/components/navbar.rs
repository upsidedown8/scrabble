use sycamore::prelude::*;

#[component]
pub fn Navbar<G: Html>(ctx: ScopeRef) -> View<G> {
    let active = ctx.create_signal(false);
    let logged_in = ctx.create_signal(false); // TODO

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
            "scrabble"
            a(class=burger_class, on:click=onclick) {
                span {}
                span {}
                span {}
            }
        }
    }
}

#[component]
fn NavbarStart<'a, G: Html>(ctx: ScopeRef<'a>, _logged_in: &'a Signal<bool>) -> View<G> {
    view! { ctx,
        div(class="navbar-start") {
            div(class="buttons") {
                a(class="button is-primary") {
                    "Play"
                }
            }
        }
    }
}

#[component]
fn NavbarEnd<'a, G: Html>(ctx: ScopeRef<'a>, logged_in: &'a Signal<bool>) -> View<G> {
    view! { ctx,
        div(class="navbar-end") {
            div(class="buttons") {
                (match *logged_in.get() {
                    true => view! { ctx,
                        a(class="button is-light", href="/account") {
                            "Account"
                        }
                        a(class="button is-primary") {
                            "Log out"
                        }
                    },
                    false => view! { ctx,
                        a(class="button is-light", href="/signup") {
                            "Sign up"
                        }
                        a(class="button is-primary", href="/login") {
                            "Log in"
                        }
                    }
                })
            }
        }
    }
}
