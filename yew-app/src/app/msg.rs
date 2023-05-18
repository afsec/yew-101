use super::model::Page;

#[derive(Debug)]
pub enum Msg {
    // NoOp,
    Increment,
    Decrement,
    ShowPage(Page),
}
