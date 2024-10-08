use std::sync::atomic::AtomicU64;
use std::sync::{Arc, RwLock};

use crate::todo::Todo;

#[derive(Clone)]
pub struct AppState {
    pub id_gen: Arc<AtomicU64>,
    pub todo_list: Arc<RwLock<Vec<Todo>>>,
}

impl AppState {
    pub fn new(todo_list: Arc<RwLock<Vec<Todo>>>) -> Self {
        Self {
            id_gen: Arc::new(AtomicU64::new(0)),
            todo_list,
        }
    }
}
