use anyhow::{anyhow, Result};
use regex::Regex;
use std::time::Duration;

pub fn parse_interval(input: &str) -> Result<Duration> {
    let re = Regex::new(r"^(?:(\d+)h)?(?:(\d+)m)?$")?;

    let caps = re.captures(input)
        .ok_or_else(|| anyhow!("Invalid interval format. Use: 1h, 30m, 1h30m, 90m"))?;

    let hours = caps.get(1)
        .map(|m| m.as_str().parse::<u64>())
        .transpose()?
        .unwrap_or(0);

    let minutes = caps.get(2)
        .map(|m| m.as_str().parse::<u64>())
        .transpose()?
        .unwrap_or(0);

    if hours == 0 && minutes == 0 {
        return Err(anyhow!("Interval must be greater than 0"));
    }

    let total_minutes = hours * 60 + minutes;
    Ok(Duration::from_secs(total_minutes * 60))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hours_only() {
        let duration = parse_interval("1h").unwrap();
        assert_eq!(duration.as_secs(), 3600);
    }

    #[test]
    fn test_parse_minutes_only() {
        let duration = parse_interval("30m").unwrap();
        assert_eq!(duration.as_secs(), 1800);
    }

    #[test]
    fn test_parse_combined() {
        let duration = parse_interval("1h30m").unwrap();
        assert_eq!(duration.as_secs(), 5400);
    }

    #[test]
    fn test_parse_minutes_as_total() {
        let duration = parse_interval("90m").unwrap();
        assert_eq!(duration.as_secs(), 5400);
    }

    #[test]
    fn test_invalid_format() {
        assert!(parse_interval("invalid").is_err());
        assert!(parse_interval("1x").is_err());
        assert!(parse_interval("0h0m").is_err());
    }
}
