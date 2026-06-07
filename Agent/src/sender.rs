use crate::system::SystemInfo;

pub async fn send_report(report: &SystemInfo) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();

    let res = client
        .post("http://127.0.0.1:8080/agent")
        .json(report)
        .send()
        .await?;

    println!("[Agent] backend status: {}", res.status());
    Ok(())
}
