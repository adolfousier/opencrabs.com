# Installation

Three ways to get OpenCrabs running.

## Option 1: Download Binary (recommended)

Grab a pre-built binary from [GitHub Releases](https://github.com/adolfousier/opencrabs/releases) — available for Linux (amd64/arm64), macOS (amd64/arm64), and Windows.

```bash
# Download and extract
curl -fsSL https://github.com/adolfousier/opencrabs/releases/latest/download/opencrabs-$(uname -m)-$(uname -s | tr A-Z a-z).tar.gz | tar xz

# Run
./opencrabs
```

The onboarding wizard handles everything on first run.

> `/rebuild` works even with pre-built binaries — it auto-clones the source to `~/.opencrabs/source/` on first use, then builds and hot-restarts.

## Option 2: Build from Source

Required for `/rebuild`, adding custom tools, or modifying the agent.

### Quick setup (recommended)

The setup script auto-detects your platform (macOS, Debian/Ubuntu, Fedora/RHEL, Arch) and installs all build dependencies + Rust nightly:

```bash
# Install all dependencies
curl -fsSL https://raw.githubusercontent.com/adolfousier/opencrabs/main/scripts/setup.sh | bash

# Clone and build
git clone https://github.com/adolfousier/opencrabs.git
cd opencrabs
cargo build --release
./target/release/opencrabs
```

### Manual setup

If you prefer to install dependencies yourself:

- **Rust nightly (2024 edition)** — [Install Rust](https://rustup.rs/), then `rustup toolchain install nightly`
- **An API key** from at least one supported provider
- **SQLite** (bundled via rusqlite)
- **macOS:** `brew install cmake pkg-config`
- **Debian/Ubuntu:** `sudo apt install build-essential pkg-config libssl-dev cmake`
- **Fedora/RHEL:** `sudo dnf install gcc gcc-c++ make pkg-config openssl-devel cmake`
- **Arch:** `sudo pacman -S base-devel pkg-config openssl cmake`

```bash
git clone https://github.com/adolfousier/opencrabs.git
cd opencrabs
cargo build --release
./target/release/opencrabs
```

> OpenCrabs uses `keys.toml` instead of `.env` for API keys. The onboarding wizard will help you set it up, or edit `~/.opencrabs/keys.toml` directly.

## Option 3: Docker

Run OpenCrabs in an isolated container. Build takes ~15min (Rust release + LTO).

```bash
git clone https://github.com/adolfousier/opencrabs.git
cd opencrabs
docker compose -f src/docker/compose.yml up --build
```

Config, workspace, and memory DB persist in a Docker volume across restarts. API keys in `keys.toml` are mounted into the container at runtime — never baked into the image.

## Autostart on Boot

Keep OpenCrabs running as a background daemon that starts with your system.

### Linux (systemd)

```bash
cat > ~/.config/systemd/user/opencrabs.service << 'EOF'
[Unit]
Description=OpenCrabs AI Agent
After=network.target

[Service]
ExecStart=%h/.cargo/bin/opencrabs daemon
Restart=on-failure
RestartSec=5
Environment=OPENCRABS_HOME=%h/.opencrabs

[Install]
WantedBy=default.target
EOF

systemctl --user daemon-reload
systemctl --user enable opencrabs
systemctl --user start opencrabs
```

Check status: `systemctl --user status opencrabs` | Logs: `journalctl --user -u opencrabs -f`

### macOS (launchd)

```bash
cat > ~/Library/LaunchAgents/com.opencrabs.agent.plist << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN"
  "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.opencrabs.agent</string>
    <key>ProgramArguments</key>
    <array>
        <string>/usr/local/bin/opencrabs</string>
        <string>daemon</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>/tmp/opencrabs.log</string>
    <key>StandardErrorPath</key>
    <string>/tmp/opencrabs.err</string>
</dict>
</plist>
EOF

launchctl load ~/Library/LaunchAgents/com.opencrabs.agent.plist
```

Update the path in `ProgramArguments` to match your install location.

### Windows (Task Scheduler)

1. `Win + R` → `taskschd.msc`
2. **Create Basic Task** → Name: `OpenCrabs`
3. Trigger: **When I log on**
4. Action: **Start a program** → `C:\Users\<you>\.cargo\bin\opencrabs.exe`, Arguments: `daemon`
5. In Properties > Settings, check **If the task fails, restart every 1 minute**

Or via PowerShell:

```powershell
$action = New-ScheduledTaskAction -Execute "$env:USERPROFILE\.cargo\bin\opencrabs.exe" -Argument "daemon"
$trigger = New-ScheduledTaskTrigger -AtLogon
$settings = New-ScheduledTaskSettingsSet -RestartCount 3 -RestartInterval (New-TimeSpan -Minutes 1)
Register-ScheduledTask -TaskName "OpenCrabs" -Action $action -Trigger $trigger -Settings $settings
```

## Updating

- **Binary users:** Type `/evolve` in the TUI to download the latest release
- **Source users:** `git pull && cargo build --release`, or type `/rebuild` in the TUI
- **Docker users:** `docker compose pull && docker compose up -d`
