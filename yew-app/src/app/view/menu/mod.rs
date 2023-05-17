use crate::app::model::Model;
use crate::app::model::Page;
use crate::app::{msg::Msg, App};
use yew::{html, html::Scope, Html};

///////////

pub struct Menu;

impl Menu {
    pub fn view(model: &Model, link: &Scope<App>) -> Html {
        html! {
            <div id={"menu"}>
                <ul>
                    <li><a href={"#"} onclick={link.callback(|_| Msg::ShowPage(Page::Home))}>{"Home"}</a></li>{"   "}
                    <li><a href={"#"} onclick={link.callback(|_| Msg::ShowPage(Page::Posts))}>{"Posts"}</a></li>{"   "}
                    <li><a href={"#"} onclick={link.callback(|_| Msg::ShowPage(Page::Settings))}>{"Settings"}</a></li>
                </ul>
            </div>
        }
    }
}

///////////
