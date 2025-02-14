use chrono::{DateTime, Local};
use croner::{errors::CronError, Cron};
use std::error::Error;

pub struct CronEntry {
    cron: Cron,
    pub command: String,
}

impl CronEntry {
    pub fn new(line: &str) -> Result<Self, Box<dyn Error>> {
        let fields: Vec<&str> = line.split_whitespace().collect();
        Ok(CronEntry {
            cron: Cron::new(&fields[0..5].join(" ")).parse()?,
            command: fields[5..].join(" "),
        })
    }

    pub fn next_occurrence(&self) -> Result<DateTime<Local>, CronError> {
        let time = Local::now();
        self.cron.find_next_occurrence(&time, false)
    }

    pub fn pattern(&self) -> String {
        self.cron.pattern.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_every_3_hour() {
        let line = "0 */3 * * * /bin/foobar";

        let entry = CronEntry::new(line).unwrap();
        assert_eq!(entry.pattern(), "0 */3 * * *");
        assert_eq!(entry.command, "/bin/foobar");
    }

    #[test]
    fn test_every_15_min() {
        let line = "*/15 * * * * /bin/foobar -a A -bee=B -see=C";

        let entry = CronEntry::new(line).unwrap();
        assert_eq!(entry.pattern(), "*/15 * * * *");
        assert_eq!(entry.command, "/bin/foobar -a A -bee=B -see=C");
    }

    #[test]
    fn test_every_7_days() {
        let line = "0 0 * * 0 ABC=abc /bin/foobar";

        let entry = CronEntry::new(line).unwrap();
        assert_eq!(entry.pattern(), "0 0 * * 0");
        assert_eq!(entry.command, "ABC=abc /bin/foobar");
    }

    #[test]
    fn test_mon_to_fri() {
        let line = "0 0 * * 1-5 echo hello world";

        let entry = CronEntry::new(line).unwrap();
        assert_eq!(entry.pattern(), "0 0 * * 1-5");
        assert_eq!(entry.command, "echo hello world");
    }
}
