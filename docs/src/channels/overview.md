# Channel Integrations

OpenCrabs connects to multiple messaging platforms simultaneously. All channels share the TUI session by default, with per-user sessions for non-owners.

## Setting Up Channels

Channels are configured through the **onboarding wizard**, not by editing TOML files manually.

### Running the Wizard

- **First launch** — the wizard runs automatically
- **Re-run** — type `/onboard` in chat, or `/onboard:channels` to jump straight to the channels step
- **Quick jump** — `/onboard:channels` opens the channel picker and returns to chat when done

### Navigation

The channel picker is a keyboard-driven TUI screen:

| Key | Action |
|-----|--------|
| `↑` / `↓` or `j` / `k` | Move focus between channels |
| `Space` | Toggle the focused channel on/off |
| `Enter` on an enabled channel | Open that channel's setup screen |
| `Enter` on **Continue** | Skip remaining setup and advance |
| `Tab` | Same as Continue — advance to the next wizard step |
| `Esc` | Go back to the previous step |

### Channel Setup Screens

When you press `Enter` on an enabled channel, a dedicated setup screen opens with the fields needed for that platform (bot token, channel ID, allowed users, etc.). Each field:

- **Auto-detects existing values** from `config.toml` / `keys.toml` (shown as masked `••••••••` for secrets, plain text for IDs)
- **Tab** moves to the next field
- **Enter** on the last field (or the Test Connection button) saves and returns to the channel list
- **BackTab** moves to the previous field

### The Five Channels

| # | Channel | Setup Fields | Test |
|---|---------|-------------|------|
| 0 | Telegram | Bot Token, Owner User ID, Respond To | Send test message |
| 1 | Discord | Bot Token, Channel ID, Allowed Users, Respond To | Send test message |
| 2 | WhatsApp | QR Code scan, Phone Allowlist | Connection status |
| 3 | Slack | Bot Token, App Token, Channel ID, Allowed Users, Respond To | Send test message |
| 4 | Trello | API Key, API Token, Board ID, Allowed Users | Board access check |

After enabling and configuring your channels, the wizard saves everything to `config.toml` and `keys.toml` automatically. You can always re-run `/onboard:channels` to modify settings.

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
| `slack_send` | 17 actions: send, reply, react, edit, delete, pin, set_topic, send_blocks, send_file (TTS voice via OGG/Opus) |
| `trello_send` | 22 actions: create_card, move_card, add_comment, add_checklist, search, etc. |

## Channel Voice Parity

All four messaging channels (Telegram, Discord, WhatsApp, Slack) now share a single code path via `crate::channels::voice::{transcribe, synthesize}`. Bot replies are recorded in the `channel_messages` table for conversation context — previously only user messages were stored.
