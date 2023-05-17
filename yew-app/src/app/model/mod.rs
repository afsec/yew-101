use yew::Properties;

#[derive(Debug, Default, Clone, Properties, PartialEq)]
pub struct Model {
    pub counter: i32,
    pub page: Page,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Page {
    #[default]
    Home,
    Posts,
    Settings,
}

impl Model {
    pub fn init() -> Self {
        Self::default()
    }
}
