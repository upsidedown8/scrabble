use seed::{prelude::*, *};

pub fn view<Msg>() -> Node<Msg> {
    div! [
        C! ["not-found-route", "columns", "is-centered", "is-vcentered", "is-flex"],
        div! [
            h1! [ "404: Not found" ],
            p! [ "An error occured: the current URL does not refer to a route" ]
        ]
    ]
}
