use yew::{html, html::Scope, Html};

use crate::app::{model::Model, msg::Msg, App};

pub struct Content;

impl Content {
    pub fn view(model: &Model, link: &Scope<App>) -> Html {
        html! {
            <div id={"content"}>
                <div id={"counter"}>
                    <button onclick={link.callback(|_| Msg::Increment)}> {"+1"} </button>
                    <p>{ model.counter }</p>
                    <button onclick={link.callback(|_| Msg::Decrement)}> {"-1"} </button>
                </div>
            </div>
        }
    }
}
