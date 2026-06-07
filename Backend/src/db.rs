use rusqlite::{params, Connection};
use crate::models::AgentReport;

pub fn init_db() -> Connection {
    let conn = Connection::open("reports.db").expect("failed to open db");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS reports (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            hostname TEXT,
            os TEXT,
            processes TEXT
        )",
        [],
    )
    .unwrap();

    conn
}

pub fn insert_report(conn: &Connection, report: &AgentReport) {
    let processes_json = serde_json::to_string(&report.processes).unwrap();

    conn.execute(
        "INSERT INTO reports (hostname, os, processes) VALUES (?1, ?2, ?3)",
        params![report.hostname, report.os, processes_json],
    )
    .unwrap();
}
