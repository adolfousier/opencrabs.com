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
- **Inline ctx budget footer** (v0.3.36) — context budget footer (`ctx: XK/YK Z% | N tok/s`) is now appended to the last response message instead of sent as a separate message. Keeps the chat clean.
- **Rolling status edit-in-place** (v0.3.36) — tool status messages (⚙️ running, ✅ done) are edited in-place instead of delete+recreate, preventing flicker and preserving scroll position.
- **Bot command hot-reload** (v0.3.36) — bot commands refresh automatically when config or skills change, without restarting the bot.
- **Guard tok/s against burst-delivery** (v0.3.36) — tok/s footer is guarded against burst-delivery artifacts so the number stays stable.
- **`follow_up_question` polish** (v0.3.34) — Telegram keyboard is now single-column with a 40-character label cap (longer options rejected with a clear error). The rolling "Running follow_up_question (16s)" status is suppressed while the keyboard is pending so buttons don't get visually buried. The LLM is instructed to call the tool silently without echoing the question text in surrounding prose
- **Inline buttons** — Provider picker, model picker, session switcher use Telegram inline keyboards
- **Image support** — Send images to the bot, receive generated images
- **Voice messages** — STT transcription + TTS response
- **All slash commands** — `/help`, `/models`, `/new`, `/sessions`, custom commands
- **Owner vs non-owner** — Owner uses the shared TUI session, non-owners get per-user sessions
- **Onboarding overhaul** (v0.3.30) — Auto-detects owner user ID from `getUpdates`, persists partial config on cancel, only Enter on the last step commits (Tab no longer silently rewrites ~30 config keys)
- **Teloxide upgrade + join detection** (v0.3.35) — Upgraded from teloxide 0.13 to 0.17. New members joining a group are now detected before the allowlist check, so the bot can greet or moderate join events. Marathon-bucket rolling status rotates through project-author quip pool for more varied status messages.

## Configuration

All Telegram options live under `[channels.telegram]` in `~/.opencrabs/config.toml`:

```toml
[channels.telegram]
enabled = true
token = "123456:ABC-DEF..."          # or store in keys.toml
allowed_users = ["123456789"]         # numeric Telegram user IDs
allowed_channels = ["-100123456"]     # restrict to specific group/channel IDs (empty = all)
respond_to = "mention"                # "all", "dm_only", "mention" (default)
session_idle_hours = 24.0             # idle timeout for non-owner sessions
rich_messages = false                 # native Telegram rich messages (Bot API 10.1)
silence_group_start = true            # silently ignore /start from non-allowed users in groups
bot_owner = ["123456789"]             # owner IDs (gated commands, /cd hidden dirs, /profiles)
```

| Field | Default | Description |
|-------|---------|-------------|
| `enabled` | `false` | Enable the Telegram bot channel |
| `token` | `None` | Telegram Bot API token from @BotFather |
| `allowed_users` | `[]` (accept all) | Numeric Telegram user IDs. Accepts int or string arrays. Empty = open mode |
| `allowed_channels` | `[]` (all channels) | Restrict bot to specific channel/group IDs. DMs always pass |
| `respond_to` | `"mention"` | When to respond in groups: `"all"` = every message, `"dm_only"` = ignore groups, `"mention"` = only when @mentioned or replied-to |
| `session_idle_hours` | `None` (no timeout) | Idle timeout in hours for non-owner sessions. Owner sessions never expire |
| `rich_messages` | `false` | Send structured replies as native Telegram rich messages (tables, headings, lists, math). Requires current mobile/desktop Telegram clients. Telegram Web and older clients show a "not supported" placeholder |
| `silence_group_start` | `true` | Silently ignore /start from non-allowed users in group chats. Users who need their ID can DM the bot |
| `bot_owner` | `[]` (first allowed_user) | Bot owner user IDs. Owners can access gated commands (/profiles, hidden files in /cd), manage profiles. Defaults to first entry in `allowed_users` |

## Per-group access control (per-chat ACL)

Telegram groups can have their own member list, so a user can be allowed in one group without gaining DM access:

- `allowed_users` (channel level) — **admins**: may DM the bot and act in any chat.
- `bot_owner` — the **owner**: always allowed everywhere.
- `[channels.telegram.groups.<chat_id>].allowed_users` — allowed in that group only. These users are refused in DMs unless they are also an admin or the owner, which closes the "DM the bot privately to escape group oversight" bypass.

DMs are gated to admins + owner. If neither `allowed_users` nor `bot_owner` is set, the bot is unconfigured and stays open (no hard lockout); set either one to lock it down.

Each group can also override `respond_to` just for itself.

```toml
[channels.telegram]
allowed_users = ["111"]                  # admins: DM + any chat
respond_to = "mention"                   # global default

[channels.telegram.groups.-1001234567890]
allowed_users = ["222", "333"]           # allowed in this group only, never via DM
respond_to = "all"                       # per-group override of the global respond_to
```

`respond_to` accepts `all`, `mention`, `dm_only`, or `auto` (reply to all while there is at most one active sender, then switch to mention-only once a second unique sender appears).

## Voice and file pickup in groups

In mention-only groups (`respond_to = "mention"`), users can share files and voice messages even when the bot isn't directly tagged in the same message:

1. **Fire-and-forget file capture** — The bot downloads ALL incoming voice, video, document, and audio files from group messages to `~/.opencrabs/tmp/`, regardless of whether the bot was mentioned. This happens silently in the background.
2. **Tag-then-ask** — A user sends a voice message, then tags the bot in a follow-up message (e.g. `@bot what did I just say?`). The bot scans the tmp directory for recent voice files from that chat (5-minute window), transcribes the most recent one, and prepends the transcript to the user's message.

This solves the core UX problem in mention-only groups: previously, tagging the bot in the same message as a voice note didn't work because Telegram sends voice and text as separate messages.

The agent can use `telegram_send` with 20+ actions. The `thread_id` field on `send` / `reply` / `send_photo` targets a specific forum topic in supergroups with topics enabled.

| Action | Description |
|--------|-------------|
| `send` | Send text message (with optional `thread_id` for forum topics) |
| `reply` | Reply to a message |
| `send_photo` | Send image file (supports `caption` and `reply_parameters` since v0.3.58) |
| `send_document` | Send document (supports `caption` and `reply_parameters` since v0.3.58) |
| `send_voice` | Send voice message |
| `list_topics` | Returns `(thread_id, topic_name)` pairs the bot has observed — translate `#announcements` into a numeric `thread_id` |
| `pin` / `unpin` | Pin or unpin a message |
| `set_reaction` | Add an emoji reaction |
| And more... | |

## Reactions (v0.3.58)

The bot understands Telegram emoji reactions in both directions:

- **Inbound** — when a user reacts to one of the bot's messages with an emoji, the bot picks up that reaction (it looks up the original bot message by its platform message ID) and can act on it as feedback.
- **Reaction-only replies** — when a short acknowledgement says it all (a thumbs-up, a 👀, a 😂), the bot can respond with just a reaction instead of a full text message, keeping the chat uncluttered.
- **Emoji validation** — only real emoji count as reaction directives; code spans and stray characters are ignored, and react directives are stripped from intermediate status messages so they never leak into the visible reply.

Use the `set_reaction` action on `telegram_send` to add a reaction to a specific message from the agent.

## Reactions (v0.3.58)

The bot understands Telegram emoji reactions in both directions:

- **Inbound** — when a user reacts to one of the bot's messages with an emoji, the bot picks up that reaction (it looks up the original bot message by its platform message ID) and can act on it as feedback.
- **Reaction-only replies** — when a short acknowledgement says it all (a thumbs-up, a 👀, a 😂), the bot can respond with just a reaction instead of a full text message, keeping the chat uncluttered.
- **Emoji validation** — only real emoji count as reaction directives; code spans and stray characters are ignored, and react directives are stripped from intermediate status messages so they never leak into the visible reply.

Use the `set_reaction` action on `telegram_send` to add a reaction to a specific message from the agent.

## Group Chat Behavior

In groups, the agent:
- Responds when mentioned by name or replied to
- Stays quiet when the conversation doesn't involve it
- Tracks context from group messages passively
