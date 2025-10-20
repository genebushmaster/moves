use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub paused: bool,
}

impl State {
    pub fn new() -> Self {
        State { paused: false }
    }

    pub fn load(path: &str) -> Result<Self> {
        if !Path::new(path).exists() {
            return Ok(State::new());
        }

        let contents = fs::read_to_string(path)?;
        let state: State = serde_json::from_str(&contents)?;
        Ok(state)
    }

    pub fn save(&self, path: &str) -> Result<()> {
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent)?;
        }

        let contents = serde_json::to_string_pretty(self)?;
        fs::write(path, contents)?;
        Ok(())
    }
}

pub fn get_state_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| String::from("/tmp"));
    Path::new(&home).join(".config/moves/state.json")
}

pub fn get_pid_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| String::from("/tmp"));
    Path::new(&home).join(".config/moves/daemon.pid")
}
