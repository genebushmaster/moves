use anyhow::{anyhow, Result};
use std::fs;
use std::path::Path;

use crate::scheduler::{run_scheduler, Mode};
use crate::state::{get_pid_path, get_state_path, State};

pub fn start_daemon(mode: Mode, interval: std::time::Duration) -> Result<()> {
    let pid_path = get_pid_path();
    let state_path = get_state_path();

    if is_running()? {
        return Err(anyhow!("Daemon is already running"));
    }

    if let Some(parent) = pid_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let state = State::new();
    state.save(state_path.to_str().unwrap())?;

    unsafe {
        let pid = libc::fork();

        if pid < 0 {
            return Err(anyhow!("Failed to fork process"));
        }

        if pid > 0 {
            fs::write(&pid_path, pid.to_string())?;
            println!("Daemon started with PID: {}", pid);
            std::process::exit(0);
        }

        libc::setsid();
    }

    let runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(async {
        run_scheduler(mode, interval, state_path.to_str().unwrap()).await
    })?;

    Ok(())
}

pub fn stop_daemon() -> Result<()> {
    let pid_path = get_pid_path();

    if !is_running()? {
        return Err(anyhow!("No daemon is running"));
    }

    let pid_str = fs::read_to_string(&pid_path)?;
    let pid: i32 = pid_str.trim().parse()?;

    unsafe {
        if libc::kill(pid, libc::SIGTERM) != 0 {
            return Err(anyhow!("Failed to send SIGTERM to daemon"));
        }
    }

    fs::remove_file(&pid_path)?;

    let state_path = get_state_path();
    if state_path.exists() {
        fs::remove_file(&state_path)?;
    }

    Ok(())
}

pub fn pause_daemon() -> Result<()> {
    if !is_running()? {
        return Err(anyhow!("No daemon is running"));
    }

    let state_path = get_state_path();
    let mut state = State::load(state_path.to_str().unwrap())?;
    state.paused = true;
    state.save(state_path.to_str().unwrap())?;

    Ok(())
}

pub fn resume_daemon() -> Result<()> {
    if !is_running()? {
        return Err(anyhow!("No daemon is running"));
    }

    let state_path = get_state_path();
    let mut state = State::load(state_path.to_str().unwrap())?;
    state.paused = false;
    state.save(state_path.to_str().unwrap())?;

    Ok(())
}

pub fn daemon_status() -> Result<String> {
    if !is_running()? {
        return Ok(String::from("Daemon is not running"));
    }

    let state_path = get_state_path();
    let state = State::load(state_path.to_str().unwrap())?;

    let status = if state.paused {
        "running (paused)"
    } else {
        "running (active)"
    };

    Ok(format!("Daemon is {}", status))
}

fn is_running() -> Result<bool> {
    let pid_path = get_pid_path();

    if !pid_path.exists() {
        return Ok(false);
    }

    let pid_str = fs::read_to_string(&pid_path)?;
    let pid: i32 = pid_str.trim().parse()?;

    let exists = Path::new(&format!("/proc/{}", pid)).exists();

    if !exists {
        fs::remove_file(&pid_path)?;
    }

    Ok(exists)
}
