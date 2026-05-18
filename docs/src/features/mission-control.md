# Mission Control

Mission Control is a full-screen TUI dashboard that brings RSI activity, inbox proposals, and scheduled jobs into one place. Open it with `/mission-control`.

## The Three Panels

The screen is divided into three panels: a large **Inbox** on the left, and **Activity** + **Schedule** stacked on the right.

### Inbox

Pending RSI proposals displayed as cards. Each card shows:
- **Tool proposals** (orange `tool` badge) — new dynamic tools RSI thinks you need, with the shell command template
- **Command proposals** (teal `command` badge) — new slash commands RSI drafted based on usage patterns
- **Skill proposals** — new `SKILL.md` files RSI drafted when it detects a repeated multi-step workflow that isn't covered by an existing skill

Each card shows the proposal name, type badge, description or command template, and how long ago it was proposed.

Apply or reject proposals inline:
- `a` — apply the selected proposal (installs tool/command to config, or creates skill directory)
- `r` — reject the selected proposal (archives with optional reason)

Applied and rejected entries are archived daily to `~/.opencrabs/rsi/applied/` and `~/.opencrabs/rsi/rejected/` so the trail is auditable.

A banner on session start shows the count of pending inbox items.

### Activity

A chronological feed of the last 100 RSI improvements. Shows what the autonomous engine did, when, and why:
- Brain file modifications (SOUL.md, MEMORY.md, TOOLS.md, etc.)
- Template syncs from upstream
- Hard rule additions
- Feedback analysis summaries
- Violation count updates

Each entry shows the time ago, a summary of the change, and the target file.

### Schedule

Your cron job queue with paused/active state. Each job shows:
- Job name
- Cron expression
- Next run time (when active)
- `paused` label (when paused)

See [Cron Jobs](./cron-jobs.md) for full cron documentation.

### Cron BLOB Recovery (v0.3.20)

Cron jobs with legacy `BLOB`-typed prompt rows in the database are now tolerated instead of causing silent failures. The schedule panel resumes showing jobs normally.

## Keyboard Navigation

| Key | Action |
|-----|--------|
| `Tab` / `Shift+Tab` | Cycle focus between panels (Inbox → Activity → Schedule) |
| `↑` / `↓` | Move selection within a panel |
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
- [Skills System](./skills.md) — skills proposed by RSI land in the inbox
- [Dynamic Tools](./dynamic-tools.md) — what RSI tool proposals install into
- [Custom Commands](../brain/commands.md) — what RSI command proposals install into
- [Cron Jobs](./cron-jobs.md) — the schedule panel
