#[derive(Debug, Clone)]
pub struct Todo {
    pub id: u64,
    pub content: String,
    pub is_done: bool,
}

impl Todo {
    pub fn new(id: u64, content: String) -> Self {
        Self {
            id,
            content,
            is_done: false,
        }
    }
}
