use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CronEntry {
    pattern: String,
    next: String,
    command: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CronKeyVal {
    pub key: String,
    pub value: CronEntry,
}
