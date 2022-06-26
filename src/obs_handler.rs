use anyhow::Result;
use obws::Client;
use tokio::time::{sleep, Duration};
use std::env;

#[tokio::main]
pub async fn recording_handler() -> Result<()> {
    let ip = "10.240.236.111";
    let port = 4444;
    let pass = env::var("OBSPASS").unwrap().to_string();
    let client = Client::connect(ip, port).await?;
    client.login(Some(pass)).await?;
    //test
    client.recording().start_recording().await?;
    sleep(Duration::from_millis(5000)).await;
    client.recording().stop_recording().await?;
    Ok(())
}

pub async fn test_connection(ip: String, port: u16, pass: String) -> Result<()> {
    let client = Client::connect(ip, port).await?;
    client.login(Some(pass)).await?;
    client.general().get_version().await?;
    Ok(())
}

