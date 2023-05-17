pub mod model;
mod msg;
pub mod update;
pub mod view;

use self::update::Update;
use self::{model::Model, view::View};
use msg::Msg;

use gloo_console::log as console_log;
use yew::{Component, Context, Html};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct App(Model);

impl Component for App {
    type Message = Msg;

    // TODO: Learn about it
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self(Model::init())
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let model = &mut self.0;
        Update::go(model, ctx, msg)
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let model = &self.0;
        console_log!(format!("{model:?}"));
        View::go(model, ctx)
    }
}
