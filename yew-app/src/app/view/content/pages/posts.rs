use yew::{html, html::Scope, Html};

use crate::app::{model::Model, App};

pub fn render(model: &Model, link: &Scope<App>) -> Html {
    html! {
        <div>
            <h1>{"Posts"}</h1>
            <div class="card w-96 bg-neutral text-neutral-content">
                <div class="card-body items-center text-center">
                    <h2 class="card-title">{"Cookies!"}</h2>
                    <p>{"We are using cookies for no reason."}</p>
                    <div class="card-actions justify-end">
                        <button class="btn btn-primary">{"Accept"}</button>
                        <button class="btn btn-ghost">{"Deny"}</button>
                        </div>
                </div>
            </div>
            <div></div>
        </div>
    }
}
