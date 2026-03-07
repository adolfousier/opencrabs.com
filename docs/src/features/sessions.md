# Sessions

OpenCrabs supports multiple concurrent sessions, each with its own conversation history, provider, model, and working directory.

## Creating Sessions

- **TUI:** Press `Ctrl+N` or type `/new`
- **Channels:** Type `/new` in any channel

## Switching Sessions

- **TUI:** Press `Ctrl+S` to open the sessions screen, navigate with arrow keys, press `Enter` to select
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
| Switch | `Ctrl+S` + Enter | `/sessions` |
| Rename | `R` on sessions screen | — |
| Delete | `D` on sessions screen | — |

## Background Processing

Sessions can process in the background while you work in another session. The sessions screen shows:
- Spinner for actively processing sessions
- `!` for sessions waiting for tool approval
- Dot for sessions with unread messages
