use axum::{
    Json,
    http::StatusCode,
};

use crate::{
    models::AgentReport,
    db::{init_db, insert_report},
};


pub async fn agent_handler(
    Json(report): Json<AgentReport>,
) -> StatusCode {
    println!("{:#?}", report);
    StatusCode::OK
}

pub async fn reports_handler() -> Json<Vec<AgentReport>> {
    Json(vec![
        AgentReport {
            hostname: "TEST-PC".to_string(),
            os: "Windows 11".to_string(),
            process_count: 120,
            uptime: 5000,
        }
    ])
}

pub async fn agents_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!([
        {
            "hostname": "TEST-PC",
            "os": "Windows 11",
            "last_seen": "Online"
        }
    ]))
}