use std::ops::{AddAssign, SubAssign};
use yew::Context;

use crate::App;

use super::{model::Model, msg::Msg};
use crate::helpers::console_log;
pub struct Update;

impl Update {
    pub fn go(model: &mut Model, _ctx: &Context<App>, msg: crate::app::Msg) -> bool {
        #[cfg(debug_assertions)]
        console_log(format!("Update: Msg::{msg:?}"));
        match msg {
            Msg::Increment => model.counter.add_assign(1),
            Msg::Decrement => model.counter.sub_assign(1),
            Msg::ShowPage(page) => model.page = page,
        };
        true
    }
}
