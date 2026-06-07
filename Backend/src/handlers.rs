use axum::{Json, http::StatusCode};
use crate::models::AgentReport;

pub async fn agent_handler(
    Json(report): Json<AgentReport>,
) -> StatusCode {
    println!("📥 Agent report received:");
    println!("{:#?}", report);

    StatusCode::OK
}
