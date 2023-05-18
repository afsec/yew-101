mod pages;

use yew::{html::Scope, Html};

use crate::app::{
    model::{Model, Page},
    App,
};

pub struct Content;

impl Content {
    pub fn view(model: &Model, link: &Scope<App>) -> Html {
        match model.page {
            Page::Home => pages::home::render(model, link),
            Page::Posts => pages::posts::render(model, link),
            Page::Settings => pages::settings::render(model, link),
        }
    }
}
