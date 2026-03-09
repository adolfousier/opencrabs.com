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

## Updating

- **Binary users:** Type `/evolve` in the TUI to download the latest release
- **Source users:** `git pull && cargo build --release`, or type `/rebuild` in the TUI
- **Docker users:** `docker compose pull && docker compose up -d`
