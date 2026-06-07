use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AgentReport {
    pub hostname: String,
    pub os: String,
    pub process_count: usize,
    pub uptime: u64,
}
