use crate::cron_utils;
use crate::etcd_service::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tracing::info;
use rand::distr::{Alphanumeric, SampleString};

const SAMPLE_LENGTH: usize = 8;

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

pub fn generate_unique_key(prefix: &str) -> String {
    format!(
        "{}/{}",
        prefix,
        Alphanumeric.sample_string(&mut rand::rng(), SAMPLE_LENGTH)
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_unique_key() {
        let prefix = "cron";

        let key = generate_unique_key(prefix);
        let parts: Vec<&str> = key.split("/").collect();
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0], prefix);
        assert_eq!(parts[1].len(), SAMPLE_LENGTH);
    }
}