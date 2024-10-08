use std::sync::{Arc, RwLock};

use self::app_state::AppState;

use {axum::routing::get, axum::Router};

mod add_todo;
mod app_state;
mod todo;
mod todo_list;

#[tokio::main]
async fn main() {
    let todo_list = Arc::new(RwLock::new(vec![]));
    // app_state
    let app_state = AppState::new(todo_list);

    let app = Router::new()
        .route("/", get(crate::get::index))
        .route(
            "/add_todo",
            get(crate::add_todo::get::add_todo).post(crate::add_todo::post::add_todo),
        )
        .route("/todo_list", get(crate::todo_list::get::todo_list))
        .route("/done/:id", get(crate::todo_list::post::done))
        .route("/undone/:id", get(crate::todo_list::post::undone))
        .with_state(app_state);

    // let app = super::chat::router(app_state.clone()).layer(auth_layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    // Ensure we use a shutdown signal to abort the deletion task.
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

mod get {
    use askama::Template;

    #[derive(Template)]
    #[template(path = "index.html")]
    pub struct IndexTemplete;

    pub async fn index() -> IndexTemplete {
        IndexTemplete
    }
}
