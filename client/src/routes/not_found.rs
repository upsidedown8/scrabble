use yew::prelude::*;

#[function_component(NotFoundRoute)]
pub fn not_found_route() -> Html {
    html! {
        <div class="not-found-route columns is-centered is-vcentered is-flex">
            <div>
                <h1>{ "404: Not found" }</h1>
                <p>{ "An error occured: the current URL does not refer to a route" }</p>
            </div>
        </div>
    }
}
