mod content;
mod menu;

use yew::{html, Context, Html};

use self::{content::Content, menu::Menu};
use super::{model::Model, App};
pub struct View;

impl View {
    pub fn go(model: &Model, ctx: &Context<App>) -> Html {
        html! {
            <div id={"main"}>
                { Menu::view(model, ctx.link()) }
                { Content::view(model, ctx.link()) }
            </div>

        }
    }
}
