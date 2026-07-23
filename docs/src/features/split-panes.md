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

## Live Background Updates (v0.3.36)

Inactive panes now update **live** in the background. Previously, non-focused panes only refreshed when you switched focus to them. Now a background-session live-state cache routes `IntermediateText` and `QueuedUserMessage` events into per-session deltas, so you can watch tool calls and responses appearing in other panes in real time without switching focus.

**Provider/model contamination prevention:** When closing or switching panes, the old session's provider is captured *before* the switch. This blocks cross-provider model leaks at 27 call sites throughout the codebase. The footer always shows the correct provider+model for the focused pane.

**Ctrl+N** binds the focused pane and live-refreshes the footer title, so new sessions show up immediately in the status bar.

## Reasoning Expand (v0.3.74)

Reasoning blocks now cycle through **three states** instead of toggling open/closed. Click a reasoning block (or press `Ctrl+O`) to cycle: **collapsed → capped → full**. The capped middle state shows a bounded preview so a long chain-of-thought never floods the viewport (#727, #726).

Two refinements make expanding predictable:

- **Anchor block on expand** — the clicked block's header is pinned to its screen row, so expanding or collapsing grows the block in place instead of jumping the viewport (#728).
- **Click vs drag on mouse-up** — click-vs-drag is decided on mouse-up, so a click-drag selects text (for copying) instead of toggling the expand state (#726).

## Version in the UI (v0.3.74)

The running OpenCrabs version now shows on the **TUI header** and in channel `/help` and `/usage` output (#696), so you always know which build a session is on.

## Double-Escape Returns the Query (v0.3.74)

Pressing `Esc` twice before any reply now **returns the query to the input box and removes it** from the conversation (#698), so a half-typed prompt you cancel doesn't linger as an empty turn.

## Limits

There is no hard limit on pane count -- you can run as many as your terminal fits. Each pane is a full session with its own token tracking and working directory.
