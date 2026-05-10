# Mission Control

Mission Control is a full-screen TUI dashboard that brings RSI activity, inbox proposals, and scheduled jobs into one place. Open it with `/mission-control`.

## The Three Panels

Mission Control has three panels in a single view:

### Inbox

Pending RSI proposals displayed as cards. Each card shows:
- **Tool proposals** — new dynamic tools RSI thinks you need, with the tool name, description, and rationale
- **Command proposals** — new slash commands RSI drafted based on usage patterns

1. **Tool/command proposals** — new dynamic tools and slash commands RSI drafted based on usage patterns
2. **Skill proposals** — new `SKILL.md` files RSI drafted when it detects a repeated multi-step workflow that isn't covered by an existing skill

Apply or reject proposals inline:
- `a` — apply the selected proposal (installs tool/command to config, or creates skill directory)
- `r` — reject the selected proposal (archives with optional reason)

Applied and rejected entries are archived daily to `~/.opencrabs/rsi/applied/` and `~/.opencrabs/rsi/rejected/` so the trail is auditable.

A banner on session start shows the count of pending inbox items.

### Activity

A chronological feed of recent RSI improvements. Shows what the autonomous engine did, when, and why:
- Brain file modifications (SOUL.md, TOOLS.md, etc.)
- Template syncs from upstream
- Feedback analysis summaries

### Schedule

Your cron job queue with paused/active state. See [Cron Jobs](./cron-jobs.md) for full cron documentation.

## Keyboard Navigation

| Key | Action |
|-----|--------|
| `Tab` / `Shift+Tab` | Cycle focus between panels (Inbox → Activity → Schedule) |
| `j` / `k` or `↑` / `↓` | Move selection within a panel |
| `g` / `Home` | Jump to top of list |
| `G` / `End` | Jump to bottom of list |
| `Enter` | Open detail popup for selected item |
| `a` | Apply selected inbox proposal |
| `r` | Reject selected inbox proposal |
| `Esc` | Close popup or exit Mission Control |

## Architecture

Mission Control is split into three module trees:

| Layer | Path | Purpose |
|-------|------|---------|
| Data services | `brain/mission_control/` | Fetches inbox proposals, activity log, schedule items |
| Panel renderers | `tui/render/mission_control/` | Draws each panel (inbox, activity, schedule, detail popup) |
| App state + input | `tui/app/mission_control/` | Focus management, keystroke handling, actions |

Layout and keystroke contracts are unit-testable without spinning up a full `App` instance.

## Related

- [Self-Improvement (RSI)](./self-improvement.md) — the engine that generates proposals
- [Dynamic Tools](./dynamic-tools.md) — what RSI tool proposals install into
- [Custom Commands](../brain/commands.md) — what RSI command proposals install into
- [Cron Jobs](./cron-jobs.md) — the schedule panel
