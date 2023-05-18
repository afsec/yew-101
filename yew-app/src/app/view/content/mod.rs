mod pages;

use yew::{html, html::Scope, Html};

use crate::app::{
    model::{Model, Page},
    App,
};

pub struct Content;

impl Content {
    pub fn view(model: &Model, link: &Scope<App>) -> Html {
        let page = match model.page {
            Page::Home => pages::home::render(model, link),
            Page::Posts => pages::posts::render(model, link),
            Page::Settings => pages::settings::render(model, link),
        };
        html! {
            <div class="flex">
                <div class="flex-1 p-4 m-2 bg-gray-200 h-full">
                    {page}
                </div>
            </div>
        }
    }
}
