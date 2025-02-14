use crate::cron_utils;
use serde::{Deserialize, Serialize};
use std::error::Error;

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

pub fn add(expression: &str) -> Result<AddResponse, Box<dyn Error>> {
    let cron = cron_utils::CronEntry::new(expression)?;

    //TODO: implement actual add
    // 1. lock
    // 2. generate a unique key
    // 3. add a new key/val to the crontab
    // 4. add the next fire-time to the jobs
    // 5. unlock
    Ok(AddResponse {
        pattern: cron.pattern(),
        next: cron.next_occurrence()?.to_string(),
        command: cron.command,
    })
}
