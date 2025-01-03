use axum::Router;
use sqlx::{Pool, Postgres};

mod chat;
mod user;

pub use chat::chat_handler;
pub use user::{get_prompt_handler, get_summary_handler, login_handler, update_prompt_handler};

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
}

pub fn create_router(pool: Pool<Postgres>) -> Router {
    let state = AppState { pool };

    Router::new()
        // user routes
        .route("/login", axum::routing::post(login_handler))
        .route(
            "/prompt/:user_id",
            axum::routing::get(get_prompt_handler).post(update_prompt_handler),
        )
        .route("/summary/:user_id", axum::routing::get(get_summary_handler))
        // chat route
        .route("/chat", axum::routing::post(chat_handler))
        // Attach state
        .with_state(state)
}
