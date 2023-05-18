use yew::{html, html::Scope, Html};

use crate::app::{model::Model, msg::Msg, App};

pub fn render(model: &Model, link: &Scope<App>) -> Html {
    html! {
        <div>
            <h1>{"Settings"}</h1>
            <div class="flex justify-center gap-2 items-center">
               <button
                    class="btn btn-outline"
                    onclick={link.callback(|_| Msg::Decrement)}>
                        {"-1"}
                </button>
                <div>{ model.counter }</div>
                <button
                    class="btn btn-outline"
                    onclick={link.callback(|_| Msg::Increment)}>
                        {"+1"}
            </button>
            </div>
        </div>
    }
}
