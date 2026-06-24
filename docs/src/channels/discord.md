# Discord

Connect OpenCrabs to Discord for server and DM interactions.

## Setup

### Step 1: Create a Discord Bot

1. Go to [discord.com/developers/applications](https://discord.com/developers/applications)
2. Create a new application
3. Go to **Bot** section, create a bot
4. **Enable MESSAGE CONTENT Intent** (required — under Privileged Gateway Intents)
5. Copy the bot token
6. Under **OAuth2 → URL Generator**, select `bot` scope with `Send Messages` and `Read Message History` permissions
7. Use the generated URL to invite the bot to your server

### Step 2: Configure via the Onboarding Wizard

Run `/onboard:channels` (or `/onboard` and navigate to the Channels step):

1. Use `↑`/`↓` to focus **Discord**
2. Press `Space` to toggle it on
3. Press `Enter` to open the Discord setup screen
4. Fill in the fields:
   - **Bot Token** — paste the token from the Developer Portal
   - **Channel ID** — the Discord channel to send the welcome message to (right-click a channel with Developer Mode on → Copy Channel ID)
   - **Allowed Users** — comma-separated Discord user IDs (leave empty to allow everyone)
   - **Respond To** — `all`, `dm_only`, or `mention`
5. Press `Enter` on **Test Connection** to verify
6. Press `Enter` to save and return to the channel list

> Enable Developer Mode in Discord: Settings → Advanced → Developer Mode

### Manual Configuration (advanced)

```toml
# keys.toml
[channels.discord]
token = "your-bot-token"

# config.toml
[channels.discord]
enabled = true
allowed_channels = ["123456789"]
allowed_users = []
respond_to = "all"
```

## Configuration

All Discord options live under `[channels.discord]` in `~/.opencrabs/config.toml`:

```toml
[channels.discord]
enabled = true
token = "your-discord-bot-token"       # or store in keys.toml
allowed_users = ["123456789012345678"]  # Discord user IDs
allowed_channels = ["123456789012345678"]
respond_to = "mention"                  # "all", "dm_only", "mention" (default)
session_idle_hours = 24.0               # idle timeout for non-owner sessions
```

| Field | Default | Description |
|-------|---------|-------------|
| `enabled` | `false` | Enable the Discord bot channel |
| `token` | `None` | Discord bot token from the Developer Portal |
| `allowed_users` | `[]` (accept all) | Discord user IDs. Accepts int or string arrays |
| `allowed_channels` | `[]` (all channels) | Restrict bot to specific channel IDs |
| `respond_to` | `"mention"` | When to respond: `"all"`, `"dm_only"`, `"mention"` |
| `session_idle_hours` | `None` (no timeout) | Idle timeout for non-owner sessions. Owner sessions never expire |

## Features

- **Server channels and DMs** — Works in text channels and direct messages
- **Button interactions** — Provider picker, model picker, session switcher use Discord buttons
- **Image support** — Send and receive images
- **Embed suppression** — Agent wraps multiple links in `<>` to suppress embeds
- **Slash commands** — All built-in and custom commands work
- **Reactions** — Agent can add emoji reactions to messages

## Formatting Notes

- No markdown tables in Discord — use bullet lists instead
- Wrap multiple links in `<url>` to suppress embeds
