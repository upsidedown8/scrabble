use yew::prelude::*;

#[function_component(LoginRoute)]
pub fn login_route() -> Html {
    html! {
        <div class="login columns">
                <form class="box">
                    <div class="field">
                        <label class="label">{ "Email" }</label>
                        <div class="control">
                            <input class="input" type="email" placeholder="e.g. alex@example.com" />
                        </div>
                    </div>

                    <div class="field">
                        <label class="label">{ "Password" }</label>
                        <div class="control">
                            <input class="input" type="password" placeholder="********" />
                        </div>
                    </div>

                    <button class="button is-primary">{ "Sign in" }</button>
                </form>
        </div>
    }
}
