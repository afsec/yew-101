pub mod model;
mod msg;
pub mod update;
pub mod view;

use self::model::Model;
use self::msg::Msg;
use self::update::Update;

use yew::{html, Component, Context, Html};

pub struct App(Model);

impl Component for App {
    type Message = Msg;

    // TODO: Learn about it
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self(Model { counter: 0 })
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let model = &mut self.0;
        Update::go(model, ctx, msg)
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <button onclick={ctx.link().callback(|_| Msg::Increment)}> {"+1"} </button>
                <p>{ self.0.counter }</p>
                <button onclick={ctx.link().callback(|_| Msg::Decrement)}> {"-1"} </button>
            </div>
        }
    }
}
