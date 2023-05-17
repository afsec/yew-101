mod content;
mod menu;

use yew::{html, Context, Html};

use self::menu::Menu;

use super::{model::Model, App};
use crate::app::{view::content::Content, Msg};

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
