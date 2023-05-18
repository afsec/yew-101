use std::ops::{AddAssign, SubAssign};

use super::{model::Model, msg::Msg};
use crate::App;
use gloo_console::log as console_log;

use yew::Context;
pub struct Update;

impl Update {
    pub fn go(model: &mut Model, _ctx: &Context<App>, msg: crate::app::Msg) -> bool {
        console_log!(format!("Update: Msg::{msg:?}"));
        match msg {
            Msg::Increment => {
                model.counter.add_assign(1);
                console_log!("plus one");
            }
            Msg::Decrement => {
                model.counter.sub_assign(1);
                console_log!("minus one");
            }
            Msg::ShowPage(page) => {
                model.page = page;
            }
        };
        true
    }
}
