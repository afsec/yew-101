use super::{model::Model, msg::Msg};
use crate::App;
use gloo_console::log as console_log;

use yew::Context;
pub struct Update;

impl Update {
    pub fn go(model: &mut Model, _ctx: &Context<App>, msg: crate::app::Msg) -> bool {
        match msg {
            Msg::Increment => {
                model.counter += 1;
                console_log!("plus one"); // Will output a string to the browser console
            }
            Msg::Decrement => {
                model.counter -= 1;
                console_log!("minus one");
            }
        };
        true
    }
}
