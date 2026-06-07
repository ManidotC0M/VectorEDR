use axum::{
    Router,
    routing::{get, post},
};

use crate::handlers::{
    agent_handler,
    reports_handler,
    agents_handler,
};

pub fn create_routes() -> Router {
    Router::new()
        .route("/agent", post(agent_handler))
        .route("/reports", get(reports_handler))
        .route("/agents", get(agents_handler))
}