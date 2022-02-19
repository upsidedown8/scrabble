use seed::{prelude::*, *};
use crate::components::board;

pub fn view<Msg>() -> Node<Msg> {
    div! [
        C! ["home-route"],

        h1! [ "HOME" ],
        board::view(&[None; 225]),
    ]
}
