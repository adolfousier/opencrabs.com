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

## Limits

There is no hard limit on pane count — you can run as many as your terminal fits. Each pane is a full session with its own token tracking and working directory.
