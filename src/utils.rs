use rand::distr::{Alphanumeric, SampleString};

const SAMPLE_LENGTH: usize = 8;

pub(crate) fn generate_unique_key(prefix: &str) -> String {
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
