use askama::Template;
#[derive(Template)]
#[template(path = "add_todo.html")]
pub struct AddTodoTemplete;

pub mod get {
    use super::AddTodoTemplete;

    pub async fn add_todo() -> AddTodoTemplete {
        AddTodoTemplete
    }
}

pub mod post {
    use axum::extract::State;
    use axum::Form;
    use serde::Deserialize;

    use crate::todo::Todo;
    use crate::AppState;

    use super::AddTodoTemplete;

    #[derive(Debug, Clone, Deserialize)]
    pub struct AddTodoForm {
        pub content: String,
    }
    pub async fn add_todo(
        State(state): State<AppState>,
        Form(add_todo): Form<AddTodoForm>,
    ) -> AddTodoTemplete {
        let Ok(mut todo_list) = state.todo_list.try_write() else {
            panic!("can't write to todo_list");
        };
        state
            .id_gen
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        todo_list.push(Todo::new(
            state.id_gen.load(std::sync::atomic::Ordering::Relaxed),
            add_todo.content,
        ));
        AddTodoTemplete
    }
}
