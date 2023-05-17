pub struct Model {
    pub counter: i32,
}

impl Model {
    pub fn init() -> Self {
        Self { counter: 0 }
    }
}
