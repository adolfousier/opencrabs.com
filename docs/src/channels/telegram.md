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
- **Forum topic routing** (v0.3.31) — In supergroups with topics enabled, the bot tracks `thread_id` through the full pipeline. Use `list_topics` action to map topic names (e.g. `#announcements`) to numeric IDs, then pass `thread_id` to `send` / `reply` / `send_photo` to route into a specific topic
- **Context-aware pre-tool status** (v0.3.31) — While a tool runs, the bot shows a live status message naming the tool, elapsed time, and either a reasoning excerpt or an anchored phrase from the user's request
- **`follow_up_question` polish** (v0.3.34) — Telegram keyboard is now single-column with a 40-character label cap (longer options rejected with a clear error). The rolling "Running follow_up_question (16s)" status is suppressed while the keyboard is pending so buttons don't get visually buried. The LLM is instructed to call the tool silently without echoing the question text in surrounding prose
- **Inline buttons** — Provider picker, model picker, session switcher use Telegram inline keyboards
- **Image support** — Send images to the bot, receive generated images
- **Voice messages** — STT transcription + TTS response
- **All slash commands** — `/help`, `/models`, `/new`, `/sessions`, custom commands
- **Owner vs non-owner** — Owner uses the shared TUI session, non-owners get per-user sessions
- **Onboarding overhaul** (v0.3.30) — Auto-detects owner user ID from `getUpdates`, persists partial config on cancel, only Enter on the last step commits (Tab no longer silently rewrites ~30 config keys)

## Agent Tools

The agent can use `telegram_send` with 20+ actions. The `thread_id` field on `send` / `reply` / `send_photo` targets a specific forum topic in supergroups with topics enabled.

| Action | Description |
|--------|-------------|
| `send` | Send text message (with optional `thread_id` for forum topics) |
| `reply` | Reply to a message |
| `send_photo` | Send image file |
| `send_document` | Send document |
| `send_voice` | Send voice message |
| `list_topics` | Returns `(thread_id, topic_name)` pairs the bot has observed — translate `#announcements` into a numeric `thread_id` |
| `pin` / `unpin` | Pin or unpin a message |
| `set_reaction` | Add an emoji reaction |
| And more... | |

## Group Chat Behavior

In groups, the agent:
- Responds when mentioned by name or replied to
- Stays quiet when the conversation doesn't involve it
- Tracks context from group messages passively
