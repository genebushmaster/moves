# moves

A simple Linux CLI daemon that provides audible movement reminders at configurable intervals.

## Features

- Two notification modes:
  - **Fixed mode**: Notifications at clock boundaries (e.g., every hour on the hour: 9:00, 10:00, 11:00)
  - **Relative mode**: Notifications at intervals from start time (e.g., every 30 minutes from now)
- Daemon runs in background
- Pause/resume notifications without stopping the daemon
- System audio notifications using alarm sound
- Simple command-line interface

## Installation

### Build from source

Requires Rust toolchain:

```bash
cargo build --release
sudo cp target/release/moves /usr/local/bin/
```

## Usage

### Start daemon

**Relative mode** - notify every 30 minutes from now:
```bash
moves -m rel -i 30m
```

**Fixed mode** - notify on the hour:
```bash
moves -m fix -i 1h
```

**Fixed mode** - notify every 30 minutes at :00 and :30:
```bash
moves -m fix -i 30m
```

### Control daemon

```bash
moves status    # Check if daemon is running
moves pause     # Pause notifications (daemon keeps running)
moves resume    # Resume notifications
moves stop      # Stop daemon completely
```

### Interval format

Supported formats: `1h`, `30m`, `1h30m`, `90m`

- Only hours and minutes (no seconds)
- Examples: `1h`, `2h`, `15m`, `45m`, `1h30m`

## How it works

- Daemon runs in background and maintains state in `~/.config/moves/`
- Uses system PulseAudio to play alarm sound
- Fixed mode calculates next clock boundary based on interval
- Relative mode uses simple repeating timer

## License

GNU General Public License
