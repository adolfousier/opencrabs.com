# Channel Integrations

OpenCrabs connects to multiple messaging platforms simultaneously. All channels share the TUI session by default, with per-user sessions for non-owners.

## Supported Channels

| Channel | Protocol | Images In | Voice In | Image Gen Out | Setup |
|---------|----------|-----------|----------|---------------|-------|
| [Telegram](./telegram.md) | Long polling | Vision pipeline | STT | Native photo | Bot token |
| [Discord](./discord.md) | WebSocket | Vision pipeline | STT | File attachment | Bot token |
| [Slack](./slack.md) | Socket Mode | Vision pipeline | STT | File upload | Bot + App token |
| [WhatsApp](./whatsapp.md) | QR pairing | Vision pipeline | STT | Native image | QR code |
| [Trello](./trello.md) | REST API | Card attachments | — | Card attachment | API key + token |

## Common Features

All messaging channels support:

- **Shared session** with TUI (owner) or per-user sessions (non-owners)
- **Slash commands** — `/help`, `/models`, `/new`, `/sessions`, custom commands
- **Inline buttons** — Provider picker, model picker, session switcher (Telegram, Discord, Slack)
- **User allowlists** — Restrict access by user ID, chat ID, or phone number
- **`respond_to` filter** — `all`, `dm_only`, or `mention` (respond only when @mentioned)

## File & Media Support

| Channel | Images (in) | Text files (in) | Documents (in) | Audio (in) | Image gen (out) |
|---------|-------------|-----------------|----------------|------------|-----------------|
| Telegram | Vision pipeline | Extracted inline | PDF note | STT | Native photo |
| WhatsApp | Vision pipeline | Extracted inline | PDF note | STT | Native image |
| Discord | Vision pipeline | Extracted inline | PDF note | STT | File attachment |
| Slack | Vision pipeline | Extracted inline | PDF note | STT | File upload |
| Trello | Card attachments → vision | Extracted inline | — | — | Card attachment |
| TUI | Paste path → vision | Paste path → inline | — | STT | `[IMG: name]` display |

Images are passed to the active model's vision pipeline if it supports multimodal input, or routed to the `analyze_image` tool (Google Gemini vision) otherwise. Text files are extracted as UTF-8 and included inline up to 8,000 characters.

## Proactive Channel Tools

The agent can send messages and take actions proactively:

| Tool | Actions |
|------|---------|
| `discord_send` | 17 actions: send, reply, react, edit, delete, pin, create_thread, send_embed, etc. |
| `slack_send` | 17 actions: send, reply, react, edit, delete, pin, set_topic, send_blocks, etc. |
| `trello_send` | 22 actions: create_card, move_card, add_comment, add_checklist, search, etc. |
