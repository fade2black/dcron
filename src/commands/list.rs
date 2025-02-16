use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::{cron_utils::CronKeyVal, etcd_service::Client, CronEntryWithNext};

#[derive(Serialize, Deserialize, Debug)]
pub struct ListResponse(Vec<CronKeyVal>);

pub async fn list() -> Result<ListResponse, Box<dyn Error>> {
    let mut client = Client::new().await?;

    let kvs = client.get_cron_jobs().await?;
    let mut jobs = vec![];

    for (k, v) in kvs.into_iter() {
        let value: CronEntryWithNext = serde_json::from_str(&v)?;
        jobs.push(CronKeyVal { key: k, value })
    }

    Ok(ListResponse(jobs))
}
