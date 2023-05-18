#[derive(Debug, Default)]
pub struct Model {
    pub counter: i32,
    pub page: Page,
}

#[derive(Debug, Default, PartialEq)]
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
