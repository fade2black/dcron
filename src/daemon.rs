use crate::{etcd_service::Client, CronEntryWithNext};
use chrono::{DateTime, Local};
use croner::Cron;
use std::error::Error;
use tokio::time;
use tracing::info;

const SLEEP_TIME: u64 = 3;

pub async fn watch() -> Result<(), Box<dyn Error>> {
    let mut client = Client::new().await?;

    loop {
        let lock_key = client.lock().await?;
        let kvs = client.get_cron_jobs().await?;

        for kv in kvs {
            process(&mut client, &kv.0, &kv.1).await?;
        }
        client.unlock(&lock_key).await?;

        time::sleep(time::Duration::from_secs(SLEEP_TIME)).await;
    }
}

async fn process(client: &mut Client, key: &str, val: &str) -> Result<(), Box<dyn Error>> {
    let mut entry: CronEntryWithNext = serde_json::from_str(val)?;
    let time = DateTime::parse_from_str(&entry.next, "%Y-%m-%d %H:%M:%S %:z")?;

    //info!("{:?}, {}", entry, time);
    if time <= Local::now() {
        info!("Firing \"{}\"", entry.command);

        let cron = Cron::new(&entry.pattern).parse()?;
        let next = cron.find_next_occurrence(&Local::now(), false)?;
        entry.next = next.to_string();

        client
            .store_cron_job(key, &serde_json::to_string(&entry)?)
            .await?;
    }

    Ok(())
}
