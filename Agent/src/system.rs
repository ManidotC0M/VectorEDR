use sysinfo::System;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct SystemInfo {
    pub hostname: String,
    pub os: String,
    pub process_count: usize,
    pub uptime: u64,
}

pub fn collect_system_info() -> SystemInfo {
    let mut system = System::new_all();
    system.refresh_all();

    let hostname = hostname::get()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();

    let os = System::name().unwrap_or_else(|| "Unknown".into());

    SystemInfo {
        hostname,
        os,
        process_count: system.processes().len(),
        uptime: System::uptime(),
    }
}
