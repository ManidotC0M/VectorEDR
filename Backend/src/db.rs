use rusqlite::{params, Connection};
use crate::models::AgentReport;

pub fn init_db() -> Connection {
    let conn = Connection::open("reports.db").unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS reports (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            hostname TEXT NOT NULL,
            os TEXT NOT NULL,
            process_count INTEGER NOT NULL,
            uptime INTEGER NOT NULL
        )",
        [],
    ).unwrap();

    conn
}

pub fn insert_report(conn: &Connection, report: &AgentReport) {
    conn.execute(
        "INSERT INTO reports (hostname, os, process_count, uptime)
         VALUES (?1, ?2, ?3, ?4)",
        params![
            report.hostname,
            report.os,
            report.process_count,
            report.uptime
        ],
    ).unwrap();
}