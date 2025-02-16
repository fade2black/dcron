use chrono::{DateTime, Local};
use croner::{errors::CronError, Cron};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct CronEntry {
    pub pattern: String,
    pub command: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CronEntryWithNext {
    pub pattern: String,
    pub next: String,
    pub command: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CronKeyVal {
    pub key: String,
    pub value: CronEntryWithNext,
}

impl CronEntry {
    pub fn new(expression: &str) -> Result<Self, Box<dyn Error>> {
        let fields: Vec<&str> = expression.split_whitespace().collect();

        Ok(CronEntry {
            pattern: fields[0..5].join(" "),
            command: fields[5..].join(" "),
        })
    }

    pub fn next_occurrence(&self) -> Result<DateTime<Local>, CronError> {
        let cron = Cron::new(&self.pattern).parse()?;
        cron.find_next_occurrence(&Local::now(), false)
    }

    pub fn to_entry_with_next(&self) -> Result<CronEntryWithNext, Box<dyn Error>> {
        Ok(CronEntryWithNext {
            pattern: self.pattern.clone(),
            command: self.command.clone(),
            next: self.next_occurrence()?.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_every_3_hour() {
        let line = "0 */3 * * * /bin/foobar";

        let entry = CronEntry::new(line).unwrap();
        assert_eq!(entry.pattern, "0 */3 * * *");
        assert_eq!(entry.command, "/bin/foobar");
    }

    #[test]
    fn test_every_15_min() {
        let line = "*/15 * * * * /bin/foobar -a A -bee=B -see=C";

        let entry = CronEntry::new(line).unwrap();
        assert_eq!(entry.pattern, "*/15 * * * *");
        assert_eq!(entry.command, "/bin/foobar -a A -bee=B -see=C");
    }

    #[test]
    fn test_every_7_days() {
        let line = "0 0 * * 0 ABC=abc /bin/foobar";

        let entry = CronEntry::new(line).unwrap();
        assert_eq!(entry.pattern, "0 0 * * 0");
        assert_eq!(entry.command, "ABC=abc /bin/foobar");
    }

    #[test]
    fn test_mon_to_fri() {
        let line = "0 0 * * 1-5 echo hello world";

        let entry = CronEntry::new(line).unwrap();
        assert_eq!(entry.pattern, "0 0 * * 1-5");
        assert_eq!(entry.command, "echo hello world");
    }
}
