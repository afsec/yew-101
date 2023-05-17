use yew::{html, Context, Html};

use crate::app::Msg;

use super::{model::Model, App};

pub struct View;

impl View {
    pub fn go(model: &Model, ctx: &Context<App>) -> Html {
        html! {
            <div>
                <button onclick={ctx.link().callback(|_| Msg::Increment)}> {"+1"} </button>
                <p>{ model.counter }</p>
                <button onclick={ctx.link().callback(|_| Msg::Decrement)}> {"-1"} </button>
            </div>
        }
    }
}
