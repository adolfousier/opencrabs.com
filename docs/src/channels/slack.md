# Slack

Connect OpenCrabs to Slack workspaces.

## Setup

1. Go to [api.slack.com/apps](https://api.slack.com/apps)
2. Create a new app
3. Enable **Socket Mode**
4. Add bot scopes: `chat:write`, `channels:history`, `groups:history`, `im:history`, `reactions:write`
5. Install to workspace
6. Copy the **Bot Token** and **App-Level Token**
7. Add to `keys.toml`:

```toml
[channels.slack]
bot_token = "xoxb-..."
app_token = "xapp-..."
```

8. Enable in `config.toml`:

```toml
[channels.slack]
enabled = true
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
