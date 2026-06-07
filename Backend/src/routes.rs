use axum::{Router, routing::post};
use crate::handlers::agent_handler;

pub fn create_routes() -> Router {
    Router::new()
        .route("/agent", post(agent_handler))
}
