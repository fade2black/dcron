use std::error::Error;
use crate::etcd_service::Client;

pub async fn delete(key: &str) -> Result<(), Box<dyn Error>> {
    let mut client = Client::new().await?;

    let lock_key = client.lock().await?;
    client.delete_cron_job(key).await?;
    client.unlock(&lock_key).await?;

    Ok(())
}