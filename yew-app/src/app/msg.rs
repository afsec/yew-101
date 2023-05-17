use super::model::Page;

pub enum Msg {
    // NoOp,
    Increment,
    Decrement,
    ShowPage(Page),
}
