use yew::{html, html::Scope, Html};

use crate::app::{model::Model, App};

pub fn render(model: &Model, link: &Scope<App>) -> Html {
    html! {
        <div>
            <h1>{"Posts"}</h1>
            <div id={"content"}></div>
        </div>
    }
}
