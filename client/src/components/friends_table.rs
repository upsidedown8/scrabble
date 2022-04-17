use crate::pages::format_datetime;
use sycamore::prelude::*;

/// Props for `FriendsTable`.
#[derive(Prop)]
pub struct FriendsTableProps {
    /// The header of the first column.
    pub from_header: String,
    /// The header of the second column,
    pub date_header: String,
    /// The rows of the table.
    pub rows: Vec<api::routes::friends::Friend>,
}

/// Component that displays a table of friends.
#[component]
pub fn FriendsTable<G: Html>(cx: Scope, props: FriendsTableProps) -> View<G> {
    let rows = View::new_fragment(
        props
            .rows
            .into_iter()
            .map(|f| view! { cx, Friend(f) })
            .collect(),
    );

    view! { cx,
        table(class="table") {
            thead {
                th { (props.from_header) }
                th { (props.date_header) }
            }
            tbody {
                (rows)
            }
        }
    }
}

/// Component that displays the table row for a friend record.
#[component]
fn Friend<G: Html>(cx: Scope, friend: api::routes::friends::Friend) -> View<G> {
    view! { cx,
        tr {
            td { (friend.username) }
            td { (format_datetime(friend.since)) }
        }
    }
}
