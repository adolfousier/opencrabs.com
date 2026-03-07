# Discord

Connect OpenCrabs to Discord for server and DM interactions.

## Setup

1. Go to [discord.com/developers/applications](https://discord.com/developers/applications)
2. Create a new application
3. Go to **Bot** section, create a bot
4. **Enable MESSAGE CONTENT Intent** (required)
5. Copy the bot token
6. Add to `keys.toml`:

```toml
[channels.discord]
bot_token = "your-bot-token"
```

7. Enable in `config.toml`:

```toml
[channels.discord]
enabled = true
```

8. Invite the bot to your server using the OAuth2 URL with `bot` scope and `Send Messages`, `Read Message History` permissions

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
