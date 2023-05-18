use yew::{html, html::Scope, Html};

use crate::app::{
    model::{Model, Page},
    msg::Msg,
    App,
};

pub struct Menu;

impl Menu {
    pub fn view(model: &Model, link: &Scope<App>) -> Html {
        html! {
            <div>
                <ul>
                    <li>
                        <a
                            class={ if model.page == Page::Home {"active"} else {""}}
                            href={"#"} onclick={link.callback(|_| Msg::ShowPage(Page::Home))}>{"Home"}</a>
                    </li>{"   "}
                    <li>
                        <a
                            class={ if model.page == Page::Posts {"active"} else {""}}
                            href={"#"} onclick={link.callback(|_| Msg::ShowPage(Page::Posts))}>{"Posts"}</a>
                    </li>{"   "}
                    <li>
                        <a
                            class={ if model.page == Page::Settings {"active"} else {""}}
                            href={"#"} onclick={link.callback(|_| Msg::ShowPage(Page::Settings))}>{"Settings"}</a>
                    </li>
                </ul>
            </div>
        }
    }
}
