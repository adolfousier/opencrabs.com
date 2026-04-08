# CLI Commands

OpenCrabs has a full CLI with 20+ subcommands for managing every aspect of the agent.

## Usage

```bash
opencrabs [COMMAND] [OPTIONS]
```

## Commands

| Command | Description |
|---------|-------------|
| `chat` (default) | Launch the TUI chat interface |
| `daemon` | Run in background (channels only, no TUI) |
| `agent` | Interactive multi-turn chat or single-message mode |
| `cron` | Manage scheduled tasks (add/list/remove/enable/disable/test) |
| `channel` | Channel management (list, doctor) |
| `memory` | Memory management (list, get, stats) |
| `session` | Session management (list, get) |
| `db` | Database management (init, stats, clear) |
| `logs` | Log management (status, view, clean, open) |
| `service` | System service management (install/start/stop/restart/status/uninstall) |
| `status` | Show agent status |
| `doctor` | Run connection health check |
| `onboard` | Run the setup wizard |
| `completions` | Generate shell completions (bash/zsh/fish/powershell) |
| `version` | Show version info |
| `!command` | **Bang operator** — Run any shell command instantly without an LLM round-trip. Output shown as system message. e.g. `!git status`, `!ls -la` |
| `/evolve` | **Auto-update** — Downloads latest release and hot-restarts. Runs automatically on startup when `[agent] auto_update = true` |

## Configuration Flags

| Flag | Default | Description |
|------|---------|-------------|
| `[agent] auto_update` | `true` | Auto-install new releases on startup and hot-restart. Set to `false` to keep the manual prompt dialog. |

## Keyboard Shortcuts (TUI)

| Shortcut | Action |
|----------|--------|
| `F12` | Toggle mouse capture on/off for native terminal text selection |

## Startup Update Prompt

When a new version is available, a centered dialog appears on the splash screen asking you to accept (Enter) or skip (Esc). Accepting triggers `/evolve` automatically. After update, the binary restarts and the splash shows the new version.

## Channel Commands

`/doctor`, `/help`, `/usage`, `/evolve`, and system commands work directly on Telegram, Discord, Slack, and WhatsApp without going through the LLM. They execute instantly and return results in the channel.

All channel command logic is centralized in `src/channels/commands.rs` (847 lines) -- a shared handler that eliminates duplicated command logic across 5 channel implementations. Each channel delegates to `try_execute_text_command()` for consistent behavior.

`/evolve` on channels now runs directly (downloads + installs the binary) without requiring an LLM round-trip. Previously it was routed through the agent.

## Chat Mode

```bash
# Default — launch TUI
opencrabs

# Same as above
opencrabs chat
```

## Agent Mode

Non-interactive mode for scripting and automation:

```bash
# Interactive multi-turn chat
opencrabs agent

# Single-message mode
opencrabs agent -m "What files changed today?"
```

## Daemon Mode

Run OpenCrabs without the TUI — useful for servers where you only need channel bots. Supports a health endpoint for monitoring.

```bash
opencrabs daemon
```

The agent processes messages from all connected channels (Telegram, Discord, Slack, WhatsApp) but without the terminal UI. Channel bots auto-reconnect on network failures with 5-second backoff.

### Health Endpoint

Add to `config.toml` to expose a health check:

```toml
[daemon]
health_port = 8080
```

Then `GET http://localhost:8080/health` returns 200 OK with JSON status. Useful for systemd watchdog, uptime monitors, or load balancers.

## Service Management

Install OpenCrabs as a system service (launchd on macOS, systemd on Linux):

```bash
opencrabs service install
opencrabs service start
opencrabs service stop
opencrabs service restart
opencrabs service status
opencrabs service uninstall
```

## Cron Management

```bash
# List all cron jobs
opencrabs cron list

# Add a new cron job
opencrabs cron add \
  --name "Daily Report" \
  --cron "0 9 * * *" \
  --tz "America/New_York" \
  --prompt "Check emails and summarize" \
  --provider anthropic \
  --model claude-sonnet-4-20250514 \
  --thinking off \
  --deliver-to telegram:123456

# Remove a cron job (accepts name or ID)
opencrabs cron remove "Daily Report"

# Enable/disable (accepts name or ID)
opencrabs cron enable "Daily Report"
opencrabs cron disable "Daily Report"
```

## TUI Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `Enter` | Send message |
| `Esc` | Cancel / dismiss |
| `Ctrl+N` | New session |
| `Ctrl+L` | Sessions screen |
| `Ctrl+K` | Clear current session |
| `Ctrl+O` | Toggle tool group collapse |
| `\|` | Split pane horizontally |
| `_` | Split pane vertically |
| `Ctrl+X` | Close focused pane |
| `Tab` | Cycle pane focus / Accept autocomplete |
| `Up/Down` | Navigate suggestions / sessions |
| `/` | Start slash command (e.g. `/help`, `/models`) |
| `:` | Start emoji picker |
