use seed::{prelude::*, *};

mod account;
mod home;
mod login;
mod not_found;
mod signup;

const ACCOUNT: &str = "account";
const LOGIN: &str = "login";
const SIGNUP: &str = "signup";

pub fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    log!(url, Page::from(url.clone()));
    
    orders
        .subscribe(Msg::UrlChanged)
        .notify(subs::UrlChanged(url.clone()));
    
    Model {
        nav_active: false,
        base_url: url.to_hash_base_url(),
        page: Page::from(url),
    }
}

pub struct Model {
    page: Page,
    nav_active: bool,
    base_url: Url,
}

#[derive(Debug)]
pub enum Page {
    Account,
    Home,
    Login,
    Signup,
    NotFound
}

impl From<Url> for Page {
    fn from(mut url: Url) -> Self {
        match url.remaining_hash_path_parts().as_slice() {
            [ACCOUNT] => Page::Account,
            [LOGIN] => Page::Login,
            [SIGNUP] => Page::Signup,
            [] => Page::Home,
            _ => Page::NotFound,
        }
    }
}

struct_urls!();
impl<'a> Urls<'a> {
    pub fn home(self) -> Url {
        self.base_url()
    }
    pub fn account(self) -> Url {
        self.base_url().add_hash_path_part(ACCOUNT)
    }
    pub fn login(self) -> Url {
        self.base_url().add_hash_path_part(LOGIN)
    }
    pub fn signup(self) -> Url {
        self.base_url().add_hash_path_part(SIGNUP)
    }
}

#[derive(Clone)]
pub enum Msg {
    UrlChanged(subs::UrlChanged),
    ToggleNavActive,
    Logout,
}

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => model.page = Page::from(url),
        Msg::ToggleNavActive => model.nav_active = !model.nav_active,
        Msg::Logout => todo!(),
    }
}

pub fn view(model: &Model) -> Vec<Node<Msg>> {
    nodes! [
        view_navbar(&model.base_url, model.nav_active, false),

        match model.page {
            Page::Account => account::view(),
            Page::Home => home::view(),
            Page::Login => login::view(),
            Page::Signup => signup::view(),
            Page::NotFound => not_found::view(),
        }
    ]
}


const IS_ACTIVE: &str = "is-active";

/// View the navbar.
fn view_navbar(base_url: &Url, active: bool, logged_in: bool) -> Node<Msg> {
    nav! [
        C! ["navbar", "is-dark", "is-fixed-top"],

        view_navbar_brand(active),

        div! [
            C! [ "navbar-menu" ],
            IF!(active => C! [ IS_ACTIVE ]),
            
            view_navbar_start(base_url),
            view_navbar_end(base_url, logged_in),
        ]
    ]
}

fn view_navbar_brand(active: bool) -> Node<Msg> {
    div! [
        C! [ "navbar-brand" ],
        "scrabble",

        a! [
            C! [ "navbar-burger", IF!(active => IS_ACTIVE) ],
            attrs! {
                At::from("role") => "button",
            },

            span! [],
            span! [],
            span! [],

            ev(Ev::Click, |_| Msg::ToggleNavActive)
        ]
    ]
}

fn view_navbar_start(base_url: &Url) -> Node<Msg> {
    div! [
        C! [ "navbar-start" ],
        div! [
            C! [ "buttons" ],
            a! [
                C! [ "button", "is-primary" ],
                "Play"
            ]
        ]
    ]
}

fn view_navbar_end(base_url: &Url, logged_in: bool) -> Node<Msg> {
    div! [
        C! [ "navbar-end" ],
        div! [
            C! [ "buttons" ],
            if logged_in {
                view_navbar_end_logged_in(base_url)
            } else {
                view_navbar_end_logged_out(base_url)
            }
        ]
    ]
}

fn view_navbar_end_logged_in(base_url: &Url) -> Vec<Node<Msg>> {
    nodes! [
        a! [
            C! [ "button", "is-light" ],
            attrs! { At::Href => Urls::new(base_url).account() },
            "Account"
        ],
        a! [
            C! [ "button", "is-primary" ],
            "Log out",
            ev(Ev::Click, |_| Msg::Logout)
        ]
    ]
}

fn view_navbar_end_logged_out(base_url: &Url) -> Vec<Node<Msg>> {
    nodes! [
        a! [
            C! [ "button", "is-light" ],
            attrs! { At::Href => Urls::new(base_url).signup() },
            "Sign up"
        ],
        a! [
            C! [ "button", "is-primary" ],
            attrs! { At::Href => Urls::new(base_url).login() },
            "Log in"
        ]
    ]
}

