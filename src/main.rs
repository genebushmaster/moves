mod daemon;
mod interval;
mod notifier;
mod scheduler;
mod state;

use anyhow::Result;
use clap::{Parser, Subcommand};
use scheduler::Mode;

#[derive(Parser)]
#[command(name = "moves")]
#[command(about = "Movement reminder CLI - audible notifications at intervals", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long, value_name = "MODE")]
    m: Option<String>,

    #[arg(short, long, value_name = "INTERVAL")]
    i: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    Stop,
    Pause,
    Resume,
    Status,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Stop) => {
            daemon::stop_daemon()?;
            println!("Daemon stopped");
        }
        Some(Commands::Pause) => {
            daemon::pause_daemon()?;
            println!("Daemon paused");
        }
        Some(Commands::Resume) => {
            daemon::resume_daemon()?;
            println!("Daemon resumed");
        }
        Some(Commands::Status) => {
            let status = daemon::daemon_status()?;
            println!("{}", status);
        }
        None => {
            let mode_str = cli.m.ok_or_else(|| anyhow::anyhow!("Mode (-m) is required"))?;
            let interval_str = cli.i.ok_or_else(|| anyhow::anyhow!("Interval (-i) is required"))?;

            let mode = match mode_str.as_str() {
                "fix" => Mode::Fixed,
                "rel" => Mode::Relative,
                _ => return Err(anyhow::anyhow!("Invalid mode. Use 'fix' or 'rel'")),
            };

            let interval = interval::parse_interval(&interval_str)?;

            daemon::start_daemon(mode, interval)?;
        }
    }

    Ok(())
}
