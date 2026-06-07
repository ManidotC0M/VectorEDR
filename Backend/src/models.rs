use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AgentReport {
    pub hostname: String,
    pub os: String,
    pub process_count: usize,
    pub uptime: u64,
}