mod content;
mod menu;

use yew::{html, Context, Html};

use self::{content::Content, menu::Menu};
use super::{model::Model, App};

use crate::helpers::console_log;

pub struct View;

impl View {
    pub fn go(model: &Model, ctx: &Context<App>) -> Html {
        console_log(format!("View: {model:?}"));
        console_log("");
        html! {
            <div id={"main"}>
                { Menu::view(model, ctx.link()) }
                { Content::view(model, ctx.link()) }
            </div>

        }
    }
}
