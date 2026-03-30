# Split Panes

OpenCrabs supports tmux-style pane splitting in the TUI. Run multiple sessions side by side, each with its own provider, model, and context — all processing in parallel.

## Splitting

| Action | Shortcut |
|--------|----------|
| Split horizontal | `\|` (pipe) |
| Split vertical | `_` (underscore) |
| Cycle focus | `Tab` |
| Close pane | `Ctrl+X` |

## How It Works

Each pane runs an independent session. You can have one pane writing code with Claude while another reviews tests with Gemini. The status bar shows `[n/total]` to indicate which pane is focused.

- **Independent providers** — Each pane can use a different AI provider and model
- **Independent context** — Conversation history is isolated per pane
- **Parallel processing** — All panes process concurrently via Tokio
- **Persistent sessions** — Each pane's session is saved to SQLite like any other session

## Example Layout

```
┌──────────────────────┬──────────────────────┐
│  Session 1 (Claude)  │  Session 2 (Gemini)  │
│  Writing code...     │  Reviewing PR...     │
├──────────────────────┴──────────────────────┤
│  Session 3 (OpenRouter)                      │
│  Running tests...                            │
└──────────────────────────────────────────────┘
```

Split vertically with `_`, then horizontally with `|` in the top pane.

## Persistent Layout

Split pane configuration (splits, sizes, focused pane) saves to `~/.opencrabs/pane_layout.json` on quit and Ctrl+C. On restart, your layout is restored exactly as you left it. Each restored pane preloads its session messages from the database, so content is visible immediately instead of blank.

## Non-Focused Panes

Non-focused panes show compact tool call summaries and stripped reasoning text. Tool groups display as single collapsed lines matching the focused pane style. All panes auto-scroll to the bottom when new messages arrive.

v0.2.92 fixed several rendering issues:
- Tool calls no longer show a perpetual "running" spinner after completion
- Scroll position correctly tracks for inactive panes
- Stale cache is cleared when sessions are updated or deleted

## State Management

Deleting a session now properly cleans up cached pane state. Previously, deleting a session left stale entries in the pane cache, which could cause phantom panes on restart.

## Limits

There is no hard limit on pane count -- you can run as many as your terminal fits. Each pane is a full session with its own token tracking and working directory.
