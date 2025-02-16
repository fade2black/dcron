use crate::cron_utils::{CronEntry, CronEntryWithNext};
use crate::etcd_service::Client;
use rand::distr::{Alphanumeric, SampleString};
use serde::{Deserialize, Serialize};
use std::error::Error;

const SAMPLE_LENGTH: usize = 8;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddRequest {
    pub expression: String,
}

pub async fn add(expression: &str) -> Result<CronEntryWithNext, Box<dyn Error>> {
    let entry = CronEntry::new(expression)?.to_entry_with_next()?;
    let json_str = serde_json::to_string(&entry)?;

    store_cron_job(&json_str).await?;

    Ok(entry)
}

async fn store_cron_job(json_str: &str) -> Result<(), Box<dyn Error>> {
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
