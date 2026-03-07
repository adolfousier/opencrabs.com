# Telegram

Connect OpenCrabs to Telegram for DMs and group chats.

## Setup

1. Message [@BotFather](https://t.me/BotFather) on Telegram
2. Create a new bot with `/newbot`
3. Copy the bot token
4. Add to `keys.toml`:

```toml
[channels.telegram]
bot_token = "123456:ABC..."
```

5. Enable in `config.toml`:

```toml
[channels.telegram]
enabled = true
owner_chat_id = 123456789  # Your Telegram user ID
```

> Get your chat ID by messaging [@userinfobot](https://t.me/userinfobot) on Telegram.

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
