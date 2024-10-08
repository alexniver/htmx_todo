use askama::Template;

use crate::todo::Todo;

#[derive(Template)]
#[template(path = "todo_list.html")]
pub struct TodoListTemplete {
    todo_list: Vec<Todo>,
}

impl TodoListTemplete {
    pub fn new(todo_list: Vec<Todo>) -> Self {
        Self { todo_list }
    }
}

pub mod get {
    use axum::extract::State;

    use crate::app_state::AppState;

    use super::TodoListTemplete;

    pub async fn todo_list(State(state): State<AppState>) -> TodoListTemplete {
        let vec = state.todo_list.read().unwrap().clone();
        TodoListTemplete::new(vec)
    }
}

pub mod post {
    use core::panic;

    use axum::extract::{Path, State};

    use crate::app_state::AppState;

    use super::TodoListTemplete;

    pub async fn done(State(state): State<AppState>, Path(id): Path<u64>) -> TodoListTemplete {
        let Ok(mut todo_list) = state.todo_list.write() else {
            panic!("try write todo_list fail");
        };
        if let Some(todo) = todo_list.iter_mut().find(|todo| todo.id == id) {
            todo.is_done = true;
        }
        TodoListTemplete::new(todo_list.clone())
    }

    pub async fn undone(State(state): State<AppState>, Path(id): Path<u64>) -> TodoListTemplete {
        let Ok(mut todo_list) = state.todo_list.write() else {
            panic!("try write todo_list fail");
        };
        if let Some(todo) = todo_list.iter_mut().find(|todo| todo.id == id) {
            todo.is_done = false;
        }
        TodoListTemplete::new(todo_list.clone())
    }
}
