# Troubleshooting

Common issues and how to fix them.

## Binary Won't Start or Crashes

### Startup Errors

Run with debug logging to see what's failing:

```bash
opencrabs -d chat
```

Logs are written to `~/.opencrabs/logs/`.

### Download a Previous Version

If the latest release crashes on your machine, download a previous working version from [GitHub Releases](https://github.com/adolfousier/opencrabs/releases):

```bash
# List all releases
gh release list -R adolfousier/opencrabs

# Download a specific version
gh release download v0.2.66 -R adolfousier/opencrabs -p "opencrabs-*$(uname -m)*$(uname -s | tr A-Z a-z)*"
```

---

## /evolve — Update & Rollback

`/evolve` downloads the latest release from GitHub and hot-swaps the binary. It has built-in safety checks:

1. **Download** — Fetches the platform-specific binary from GitHub Releases
2. **Pre-swap health check** — Runs `opencrabs health-check` on the new binary (10s timeout). If it fails, the new binary is deleted and your current version stays untouched.
3. **Backup** — Creates a backup at `<binary-path>.evolve_backup`
4. **Atomic swap** — Replaces the current binary
5. **Post-swap health check** — Verifies the swapped binary works. If it fails, **auto-rolls back** to the backup.
6. **Restart** — exec()-restarts into the new version

### If /evolve Fails

The most common reason is the health check caught an issue — your current version stays safe. If something went wrong after the swap:

```bash
# Restore the backup manually
cp /path/to/opencrabs.evolve_backup /path/to/opencrabs
chmod +x /path/to/opencrabs
```

### Cargo Install Fallback

When `/evolve` uses `cargo install` (building from source), it tries the **stable** toolchain first. If that fails, it automatically falls back to `cargo +nightly`. The progress message shows which toolchain succeeded.

### Check-Only Mode

The agent can check for updates without installing:

```
/evolve check_only=true
```

---

## Bash Tool Safety

The bash tool includes a **hard command blocklist** that prevents catastrophic commands even if accidentally approved:

- `rm -rf /`, `sudo rm -rf .`
- `mkfs`, `dd` to `/dev/`
- Fork bombs
- `/etc` overwrites, `/proc` writes
- Sensitive file exfiltration
- Crypto mining commands

These are blocked at the tool level — no configuration needed.

---

## Older CPUs (Pre-2011 / No AVX)

Some features require AVX/AVX2 instructions. Since v0.2.67, OpenCrabs **detects CPU capabilities at runtime** and automatically hides unavailable options in the onboarding wizard.

### What's Affected

| Feature | CPU Requirement | Fallback |
|---------|----------------|----------|
| Local embeddings (memory search) | AVX (Sandy Bridge 2011+) | FTS-only keyword search (still works) |
| Local STT (rwhisper/candle) | AVX2 (Haswell 2013+) | API mode (Groq Whisper) or disabled |
| Local TTS (Piper) | None — tested on 2007 iMac | Works on any x86/ARM CPU |

### Symptoms

- Local STT option doesn't appear in `/onboard:voice` — your CPU lacks AVX2
- Local TTS (Piper) should always be available — no CPU restrictions, works on machines as old as 2007
- Memory search falls back to text-only FTS silently
- Crash with "illegal instruction" on very old CPUs

### Fix: Build from Source with CPU Targeting

```bash
# For your specific CPU (best performance)
RUSTFLAGS="-C target-cpu=native" cargo build --release

# For Sandy Bridge (AVX but no AVX2)
RUSTFLAGS="-C target-cpu=sandybridge" cargo build --release
```

### macOS with Apple Silicon

Local STT uses **Metal GPU acceleration** on macOS — no CPU flags needed. Works out of the box on M1/M2/M3/M4.

---

## Config Issues

### Config Won't Load

If `config.toml` has a syntax error, OpenCrabs will fail to start. Restore from backup:

```bash
# Check if a backup exists
ls ~/.opencrabs/config.toml.backup

# Restore it
cp ~/.opencrabs/config.toml.backup ~/.opencrabs/config.toml
```

Or reinitialize with defaults:

```bash
opencrabs init --force
```

> **Warning:** `--force` overwrites your config. Back up `keys.toml` first — it contains your API keys.

### Manual Backup

Always keep a backup of your critical files:

```bash
cp ~/.opencrabs/config.toml ~/.opencrabs/config.toml.backup
cp ~/.opencrabs/keys.toml ~/.opencrabs/keys.toml.backup
cp ~/.opencrabs/commands.toml ~/.opencrabs/commands.toml.backup
```

---

## Channel Issues

### Telegram

**Bot not responding:**
1. Verify token from @BotFather is in `keys.toml`
2. Check your numeric user ID is in `allowed_users`
3. If `respond_to = "mention"`, you must @mention the bot in groups

**Regenerate bot token:**
1. Open @BotFather on Telegram
2. `/mybots` → select bot → API Token → Revoke
3. Copy new token to `keys.toml` under `[channels.telegram]`
4. Restart OpenCrabs

**Re-setup from scratch:** Run `/onboard:channels` in the TUI.

### WhatsApp

**QR code / session expired:**

WhatsApp sessions are stored at `~/.opencrabs/whatsapp/session.db`. To reconnect:

```bash
# Delete the session file
rm ~/.opencrabs/whatsapp/session.db

# Re-pair via onboarding
opencrabs chat --onboard
```

Or press `R` on the WhatsApp onboarding screen to reset and get a fresh QR code.

**Messages not received:**
- Verify phone number is in `allowed_phones` using E.164 format: `"+15551234567"`
- Empty `allowed_phones = []` means accept from everyone

### Discord

**Bot not receiving messages:**
1. Ensure **Message Content Intent** is enabled in Discord Developer Portal → Bot settings
2. Required intents: `gateway`, `guild_messages`, `direct_messages`, `message_content`
3. Use the **bot token** (starts with `MTk...`), not the application ID

**Regenerate token:** Discord Developer Portal → Bot → Regenerate Token

### Slack

**Both tokens required:**
- **Bot token** (`xoxb-...`): For sending messages
- **App token** (`xapp-...`): For Socket Mode (receiving events)

Without the app token, the bot can send but not receive messages.

**Socket Mode:** Must be enabled in app settings → Features → Socket Mode → ON

### Trello

**Setup:**
1. Get API key: [trello.com/app-key](https://trello.com/app-key)
2. Generate token from the same page
3. Add `board_ids` to config — the bot only monitors listed boards
4. Set `poll_interval_secs > 0` to enable polling (default 0 = disabled)

### General: Re-run Channel Setup

For any channel issues, re-run the onboarding wizard:

```bash
opencrabs chat --onboard
```

Or type `/onboard:channels` in the TUI.

---

## Local STT (Speech-to-Text)

Since v0.2.67, local STT uses **rwhisper** (candle, pure Rust) instead of whisper-rs/ggml. On macOS, it uses **Metal GPU acceleration** automatically.

### Models

| Model | Size | Quality |
|-------|------|---------|
| `quantized-tiny` | ~42 MB | Good for short messages |
| `base-en` | ~142 MB | Better accuracy (English) |
| `small-en` | ~466 MB | High accuracy |
| `medium-en` | ~1.5 GB | Best accuracy |

Models download automatically from HuggingFace on first use.

### Common Issues

**Local STT option not showing in wizard:** Your CPU lacks AVX2. Use API mode (Groq Whisper) instead, or build from source with `RUSTFLAGS="-C target-cpu=native"`.

**"No audio samples decoded":** Audio file is corrupt or unsupported format. Supported: OGG/Opus, WAV.

**Transcription hangs:** Times out after 300 seconds. Try a smaller model (`quantized-tiny`).

**Model download fails:** Check network connection. Models are fetched from HuggingFace.

**Audio too short:** Messages under 1 second are automatically padded to prevent tensor errors.

### Disabling

```toml
[voice]
stt_enabled = false
```

---

## Local TTS (Text-to-Speech)

### Requirements

- **Python 3** must be installed and in PATH
- Piper installs automatically in a venv at `~/.opencrabs/models/piper/venv/`

### Voices

| Voice | Description | Size |
|-------|-------------|------|
| `ryan` | US Male (default) | ~200-400 MB |
| `amy` | US Female | ~200-400 MB |
| `lessac` | US Female | ~200-400 MB |
| `kristin` | US Female | ~200-400 MB |
| `joe` | US Male | ~200-400 MB |
| `cori` | UK Female | ~200-400 MB |

### Common Issues

**"python3 -m venv failed":** Install Python 3. On Ubuntu: `sudo apt install python3 python3-venv`. On macOS: `brew install python3`.

**"pip install piper-tts failed":** Network issue or pip corrupted. Fix pip first:
```bash
python3 -m pip install --upgrade pip
```

**Telegram voice messages show no waveform:** This was fixed in v0.2.64 — audio is now properly encoded as OGG/Opus (RFC 7845). Update to latest version.

**Voice preview not playing:** Preview uses `afplay` (macOS), `aplay` (Linux), or `powershell` (Windows). Ensure audio output is available.

### Re-setup

Run `/onboard:voice` in the TUI to reconfigure STT/TTS mode and re-download models.

### Disabling

```toml
[voice]
tts_mode = "off"
```

---

## Local Embeddings (Memory Search)

The memory search engine uses a ~300 MB embedding model (llama.cpp) for semantic search. It requires **AVX** on x86 CPUs.

### Fallback

If embeddings can't initialize (no AVX, download failed, disk full), memory search falls back to **FTS-only** (keyword matching). It still works, just less semantic.

### Fix for Older CPUs

Build from source with CPU targeting (see [Older CPUs](#older-cpus-pre-2011--no-avx) above).

### Model Location

Models are stored in `~/.local/share/opencrabs/models/` (platform-specific data directory).

---

## Database Issues

### Location

- **Main database:** `~/.opencrabs/opencrabs.db` (SQLite + WAL)
- **WhatsApp session:** `~/.opencrabs/whatsapp/session.db`

### Database Corruption

SQLite with WAL mode is very resilient, but if corruption occurs:

```bash
# Back up the corrupted file first
cp ~/.opencrabs/opencrabs.db ~/.opencrabs/opencrabs.db.corrupt

# Reinitialize (WARNING: loses all history)
opencrabs db init
```

### Migration Errors

The database automatically migrates on startup (11 migrations). If migrating from an older version with sqlx, the transition is handled automatically — no manual steps needed.

---

## Building from Source

### Quick Setup

```bash
curl -fsSL https://raw.githubusercontent.com/adolfousier/opencrabs/main/scripts/setup.sh | bash
git clone https://github.com/adolfousier/opencrabs.git && cd opencrabs
cargo build --release
```

### Build Without Voice (Smaller Binary)

```bash
cargo build --release --no-default-features --features telegram,whatsapp,discord,slack,trello
```

### Feature Flags

| Flag | Default | Description |
|------|---------|-------------|
| `local-stt` | On | whisper.cpp for local speech-to-text |
| `local-tts` | On | Piper for local text-to-speech |
| `telegram` | On | Telegram channel |
| `whatsapp` | On | WhatsApp channel |
| `discord` | On | Discord channel |
| `slack` | On | Slack channel |
| `trello` | On | Trello channel |

---

## Debug Mode

Run with `-d` for verbose logging:

```bash
opencrabs -d chat
```

Logs go to `~/.opencrabs/logs/` with 7-day retention.
