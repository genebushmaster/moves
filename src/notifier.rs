use anyhow::{anyhow, Result};
use std::process::Command;

const DEFAULT_SOUND: &str = "/usr/share/sounds/freedesktop/stereo/alarm-clock-elapsed.oga";

pub fn play_notification() -> Result<()> {
    let output = Command::new("paplay")
        .arg(DEFAULT_SOUND)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("Failed to play sound: {}", stderr));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sound_file_exists() {
        use std::path::Path;
        assert!(Path::new(DEFAULT_SOUND).exists());
    }
}
