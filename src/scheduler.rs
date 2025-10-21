use anyhow::Result;
use chrono::{DateTime, Local, Timelike};
use std::time::Duration;
use tokio::time::{sleep, sleep_until, Instant};

use crate::notifier::play_notification;
use crate::state::State;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Fixed,
    Relative,
}

pub async fn run_scheduler(mode: Mode, interval: Duration, state_path: &str) -> Result<()> {
    match mode {
        Mode::Fixed => run_fixed_scheduler(interval, state_path).await,
        Mode::Relative => run_relative_scheduler(interval, state_path).await,
    }
}

async fn run_fixed_scheduler(interval: Duration, state_path: &str) -> Result<()> {
    loop {
        let next_time = calculate_next_fixed_time(interval)?;

        loop {
            let now = Local::now();

            if now >= next_time {
                let time_past = (now - next_time).num_seconds();

                if time_past <= 5 {
                    let state = State::load(state_path)?;
                    if !state.paused {
                        play_notification()?;
                    }
                }
                break;
            }

            let time_until_next = (next_time - now).to_std().unwrap_or(Duration::from_secs(1));
            let sleep_duration = time_until_next.min(Duration::from_secs(30));
            sleep(sleep_duration).await;
        }
    }
}

async fn run_relative_scheduler(interval: Duration, state_path: &str) -> Result<()> {
    let mut next_notification = Local::now() + chrono::Duration::from_std(interval)?;

    loop {
        let now = Local::now();

        if now >= next_notification {
            let time_past = (now - next_notification).num_seconds();

            if time_past <= 5 {
                let state = State::load(state_path)?;
                if !state.paused {
                    play_notification()?;
                }
            }
            next_notification = Local::now() + chrono::Duration::from_std(interval)?;
        }

        let time_until_next = (next_notification - now).to_std().unwrap_or(Duration::from_secs(1));
        let sleep_duration = time_until_next.min(Duration::from_secs(30));

        sleep(sleep_duration).await;
    }
}

fn calculate_next_fixed_time(interval: Duration) -> Result<DateTime<Local>> {
    let now = Local::now();
    let interval_minutes = interval.as_secs() / 60;

    let current_minutes = now.hour() as u64 * 60 + now.minute() as u64;

    let next_boundary = ((current_minutes / interval_minutes) + 1) * interval_minutes;

    let next_hour = (next_boundary / 60) % 24;
    let next_minute = next_boundary % 60;

    let mut next_time = now
        .with_hour(next_hour as u32)
        .and_then(|t| t.with_minute(next_minute as u32))
        .and_then(|t| t.with_second(0))
        .and_then(|t| t.with_nanosecond(0))
        .ok_or_else(|| anyhow::anyhow!("Failed to calculate next time"))?;

    if next_time <= now {
        next_time = next_time + chrono::Duration::days(1);
    }

    Ok(next_time)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_next_fixed_time_hour() {
        let interval = Duration::from_secs(3600);
        let next = calculate_next_fixed_time(interval).unwrap();
        assert_eq!(next.minute(), 0);
        assert_eq!(next.second(), 0);
    }

    #[test]
    fn test_calculate_next_fixed_time_30min() {
        let interval = Duration::from_secs(1800);
        let next = calculate_next_fixed_time(interval).unwrap();
        assert!(next.minute() == 0 || next.minute() == 30);
        assert_eq!(next.second(), 0);
    }
}
