# CLI Commands

OpenCrabs supports several CLI modes.

## Usage

```bash
opencrabs [COMMAND] [OPTIONS]
```

## Commands

| Command | Description |
|---------|-------------|
| `chat` (default) | Launch the TUI chat interface |
| `daemon` | Run in background (channels only, no TUI) |
| `cron` | Manage scheduled tasks |

## Chat Mode

```bash
# Default — launch TUI
opencrabs

# Same as above
opencrabs chat
```

## Daemon Mode

Run OpenCrabs without the TUI — useful for servers where you only need channel bots.

```bash
opencrabs daemon
```

The agent processes messages from all connected channels (Telegram, Discord, Slack, WhatsApp) but without the terminal UI.

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
| `Tab` | Accept autocomplete |
| `Up/Down` | Navigate suggestions / sessions |
| `/` | Start slash command (e.g. `/help`, `/models`) |
| `:` | Start emoji picker |
