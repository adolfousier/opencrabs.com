# Telegram

Connect OpenCrabs to Telegram for DMs and group chats.

## Setup

### Step 1: Create a Bot with BotFather

1. Message [@BotFather](https://t.me/BotFather) on Telegram
2. Send `/newbot` and follow the prompts
3. Copy the bot token (format: `123456:ABC-DEF...`)

### Step 2: Configure via the Onboarding Wizard

Run `/onboard:channels` (or `/onboard` and navigate to the Channels step):

1. Use `↑`/`↓` to focus **Telegram**
2. Press `Space` to toggle it on
3. Press `Enter` to open the Telegram setup screen
4. Fill in the fields:
   - **Bot Token** — paste the token from BotFather
   - **Owner User ID** — your numeric Telegram chat ID
   - **Respond To** — `all`, `dm_only`, or `mention` (when to respond in groups)
5. Press `Enter` on **Test Connection** to verify the bot works
6. Press `Enter` to save and return to the channel list

> Get your chat ID by messaging [@userinfobot](https://t.me/userinfobot) on Telegram.

### Manual Configuration (advanced)

If you prefer editing files directly, the wizard writes to `~/.opencrabs/keys.toml` and `~/.opencrabs/config.toml`:

```toml
# keys.toml
[channels.telegram]
bot_token = "123456:ABC..."

# config.toml
[channels.telegram]
enabled = true
allowed_users = ["123456789"]
respond_to = "all"
```

## Features

- **DMs and groups** — Works in private chats and group conversations
- **Inline buttons** — Provider picker, model picker, session switcher use Telegram inline keyboards
- **Image support** — Send images to the bot, receive generated images
- **Voice messages** — STT transcription + TTS response
- **All slash commands** — `/help`, `/models`, `/new`, `/sessions`, custom commands
- **Owner vs non-owner** — Owner uses the shared TUI session, non-owners get per-user sessions

## Agent Tools

The agent can use `telegram_send` with 19 actions:

| Action | Description |
|--------|-------------|
| `send_message` | Send text message |
| `send_image` | Send image file |
| `send_document` | Send document |
| `send_voice` | Send voice message |
| `get_updates` | Get recent messages |
| `pin_message` | Pin a message |
| And more... | |

## Group Chat Behavior

In groups, the agent:
- Responds when mentioned by name or replied to
- Stays quiet when the conversation doesn't involve it
- Tracks context from group messages passively
