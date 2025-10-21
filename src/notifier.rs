use anyhow::{anyhow, Result};
use std::path::PathBuf;
use std::process::Command;

fn get_sound_path() -> PathBuf {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()));

    if let Some(dir) = exe_dir {
        let custom_sound = dir.join("sounds/notification.wav");
        if custom_sound.exists() {
            return custom_sound;
        }
    }

    let cwd_sound = PathBuf::from("sounds/notification.wav");
    if cwd_sound.exists() {
        return cwd_sound;
    }

    PathBuf::from("/usr/share/sounds/freedesktop/stereo/alarm-clock-elapsed.oga")
}

pub fn play_notification() -> Result<()> {
    let sound_path = get_sound_path();

    let output = Command::new("paplay")
        .arg(&sound_path)
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
    fn test_sound_path_logic() {
        let sound_path = get_sound_path();
        assert!(sound_path.to_str().is_some());
    }
}
