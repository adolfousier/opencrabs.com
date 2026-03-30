# Sessions

OpenCrabs supports multiple concurrent sessions, each with its own conversation history, provider, model, and working directory.

## Creating Sessions

- **TUI:** Press `Ctrl+N` or type `/new`
- **Channels:** Type `/new` in any channel

## Switching Sessions

- **TUI:** Press `Ctrl+L` to open the sessions screen, navigate with arrow keys, press `Enter` to select
- **Channels:** Type `/sessions` to see recent sessions with inline buttons

## Session Screen

The sessions screen shows:
- Session name
- Created date
- Provider/model badge
- Working directory
- Token usage
- Context window usage (current session)
- Status indicators (processing spinner, pending approval, unread)

## Per-Session State

Each session remembers:
- **Provider and model** — Switch to Claude in one, Gemini in another
- **Working directory** — `/cd` persists per session
- **Conversation history** — Full message history in SQLite
- **Token count and cost** — Cumulative usage tracking

## Session Management

| Action | TUI | Channels |
|--------|-----|----------|
| New | `Ctrl+N` / `/new` | `/new` |
| Switch | `Ctrl+L` + Enter | `/sessions` |
| Rename | `R` on sessions screen | — |
| Delete | `D` on sessions screen | — |

## Background Processing

Sessions can process in the background while you work in another session. The sessions screen shows:
- Spinner for actively processing sessions
- `!` for sessions waiting for tool approval
- Dot for sessions with unread messages

## Split Panes

Run multiple sessions side by side with tmux-style pane splitting. See [Split Panes](./split-panes.md) for details.

## State Management

v0.2.92 improved session state tracking:

- **Session reload after cancellation** -- After Esc+Esc cancel, session context reloads from DB to pick up any changes made during the cancelled operation
- **Cached state cleanup** -- Deleting a session now clears stale pane cache entries, preventing phantom state on restart
- **CLI tool segment persistence** -- Tool results from CLI providers (Claude CLI, OpenCode CLI) are now saved to DB alongside regular messages, preserving correct text/tool interleaving across restarts
- **Case-insensitive tool input** -- Tool input descriptions use case-insensitive key lookup, fixing failures when providers return different casing

## Channel Sessions

All channels (Telegram, Discord, Slack, WhatsApp, Trello) now persist sessions in SQLite by channel/group title. Sessions survive process restarts -- no more lost context after daemon restart. Each channel group gets its own isolated session, while owner DMs share the TUI session.
