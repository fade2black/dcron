use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::etcd_service::Client;
use crate::shared::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ListResponse(Vec<CronKeyVal>);

pub async fn list() -> Result<ListResponse, Box<dyn Error>> {
    let mut client = Client::new().await?;

    let kvs = client.get_cron_jobs("cron").await?;
    let mut jobs = vec![];

    for (k, v) in kvs.into_iter() {
        let entry: CronEntry = serde_json::from_str(&v)?;
        jobs.push(CronKeyVal {
            key: k,
            value: entry,
        })
    }

    Ok(ListResponse(jobs))
}
