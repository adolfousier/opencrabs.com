# Slack

Connect OpenCrabs to Slack workspaces.

## Setup

### Step 1: Create a Slack App

1. Go to [api.slack.com/apps](https://api.slack.com/apps)
2. Create a new app (From Scratch)
3. Enable **Socket Mode** under Settings
4. Generate an **App-Level Token** (Settings → Basic Information → App-Level Tokens) with `connections:write` scope
5. Under **OAuth & Permissions**, add bot scopes: `chat:write`, `channels:history`, `groups:history`, `im:history`, `reactions:write`
6. Install the app to your workspace
7. Copy the **Bot Token** (`xoxb-...`) and **App-Level Token** (`xapp-...`)

### Step 2: Configure via the Onboarding Wizard

Run `/onboard:channels` (or `/onboard` and navigate to the Channels step):

1. Use `↑`/`↓` to focus **Slack**
2. Press `Space` to toggle it on
3. Press `Enter` to open the Slack setup screen
4. Fill in the fields:
   - **Bot Token** — the `xoxb-...` token
   - **App Token** — the `xapp-...` token
   - **Channel ID** — right-click a channel → View channel details → copy the Channel ID at the bottom
   - **Allowed Users** — comma-separated Slack user IDs (Profile → ⋯ → Copy member ID)
   - **Respond To** — `all`, `dm_only`, or `mention`
5. Press `Enter` on **Test Connection** to verify
6. Press `Enter` to save

### Manual Configuration (advanced)

```toml
# keys.toml
[channels.slack]
bot_token = "xoxb-..."
app_token = "xapp-..."

# config.toml
[channels.slack]
enabled = true
allowed_channels = ["C12345678"]
allowed_users = []
respond_to = "all"
```

## Configuration

All Slack options live under `[channels.slack]` in `~/.opencrabs/config.toml`:

```toml
[channels.slack]
enabled = true
token = "xoxb-your-bot-token"          # or store in keys.toml
app_token = "xapp-your-app-token"      # Socket Mode token
allowed_users = ["U12345678"]           # Slack user IDs
allowed_channels = ["C12345678"]
respond_to = "mention"                  # "all", "dm_only", "mention" (default)
session_idle_hours = 24.0               # idle timeout for non-owner sessions
```

| Field | Default | Description |
|-------|---------|-------------|
| `enabled` | `false` | Enable the Slack bot channel |
| `token` | `None` | Bot token (`xoxb-...`) |
| `app_token` | `None` | App-level token for Socket Mode (`xapp-...`) |
| `allowed_users` | `[]` (accept all) | Slack user IDs (`U12345678`) |
| `allowed_channels` | `[]` (all channels) | Restrict bot to specific channel IDs |
| `respond_to` | `"mention"` | When to respond: `"all"`, `"dm_only"`, `"mention"` |
| `session_idle_hours` | `None` (no timeout) | Idle timeout for non-owner sessions. Owner sessions never expire |

## Features

- **Channels and DMs** — Works in public/private channels and direct messages
- **Action buttons** — Provider picker, model picker, session switcher use Slack action buttons
- **Thread support** — Responds in threads when appropriate
- **Slash commands** — All built-in and custom commands work
- **Reactions** — Agent can add emoji reactions
- **TTS voice replies** — Voice responses sent as OGG/Opus files via `files.upload` with inline waveform UI

## Socket Mode

Slack uses Socket Mode (WebSocket) instead of HTTP webhooks — no public URL or ngrok needed. The connection is outbound from your machine.
