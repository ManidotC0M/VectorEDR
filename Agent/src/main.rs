mod system;
mod sender;

use system::collect_system_info;
use sender::send_report;

#[tokio::main]
async fn main() {
    println!("[Agent] collecting system info...");
    let report = collect_system_info();

    match send_report(&report).await {
        Ok(_) => println!("[Agent] report sent successfully"),
        Err(e) => eprintln!("[Agent] failed to send report: {}", e),
    }
}
