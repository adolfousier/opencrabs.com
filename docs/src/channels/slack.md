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

## Features

- **Channels and DMs** — Works in public/private channels and direct messages
- **Action buttons** — Provider picker, model picker, session switcher use Slack action buttons
- **Thread support** — Responds in threads when appropriate
- **Slash commands** — All built-in and custom commands work
- **Reactions** — Agent can add emoji reactions
- **TTS voice replies** — Voice responses sent as OGG/Opus files via `files.upload` with inline waveform UI

## Socket Mode

Slack uses Socket Mode (WebSocket) instead of HTTP webhooks — no public URL or ngrok needed. The connection is outbound from your machine.
