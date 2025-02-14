use crate::cron_utils;
use crate::etcd_service::Client;
use crate::utils::generate_unique_key;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tracing::info;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddRequest {
    pub expression: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddResponse {
    pub pattern: String,
    pub next: String,
    pub command: String,
}

pub async fn add(expression: &str) -> Result<AddResponse, Box<dyn Error>> {
    let cron = cron_utils::CronEntry::new(expression)?;
    let resp = AddResponse {
        pattern: cron.pattern(),
        next: cron.next_occurrence()?.to_string(),
        command: cron.command,
    };

    let json_str = serde_json::to_string(&resp)?;
    info!("Adding to etcd {json_str}");
    create_cron_job(&json_str).await?;

    Ok(resp)
}

async fn create_cron_job(json_str: &str) -> Result<(), Box<dyn Error>> {
    let mut client = Client::new().await?;

    let lock_key = client.lock().await?;
    let key = generate_unique_key("cron");
    client.store_cron_job(&key, json_str).await?;
    client.unlock(&lock_key).await?;

    Ok(())
}
