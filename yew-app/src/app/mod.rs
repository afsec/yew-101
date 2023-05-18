pub mod model;
mod msg;
pub mod update;
pub mod view;

use self::{model::Model, msg::Msg, update::Update, view::View};

use yew::{Component, Context, Html};

#[derive(Debug, Default)]
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
        View::go(model, ctx)
    }
}
