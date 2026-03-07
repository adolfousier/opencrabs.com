# Changelog

For the full changelog, see [CHANGELOG.md on GitHub](https://github.com/adolfousier/opencrabs/blob/main/CHANGELOG.md).

## Recent Releases

### v0.2.59 (2026-03-07)

**Added:**
- Fallback provider chain — Configure multiple fallback providers tried in sequence on failure
- Per-provider `vision_model` — Auto-swap model when images are present
- Session working directory persistence — `/cd` changes persist to DB, restored on session switch

**Fixed:**
- Update checker uses proper semver comparison instead of string inequality
- Home directory collapsed to `~/...` in TUI footer and help screen

### v0.2.58 (2026-03-07)

**Fixed:**
- Vision images in OpenAI-compatible providers — `ContentBlock::Image` was silently dropped. Changed `OpenAIMessage.content` to `serde_json::Value` for polymorphic content support

**Docs:**
- Image & file handling in brain templates — Added `<<IMG:path>>` documentation to AGENTS.md and TOOLS.md

### v0.2.57 (2026-03-07)

**Added:**
- Two-step `/models` flow — Provider picker first, then model picker
- `/new` and `/sessions` commands for all channels
- User-defined slash commands on channels
- Custom commands visible in `/help`
- Emoji picker in TUI (`:` trigger)
- VOICE.md template

**Fixed:**
- Context counter accuracy — System brain tokens now counted
- Stream bleed between sessions
- Session switch shows name instead of UUID

### v0.2.56 (2026-03-06)

**Added:**
- Daily release check notification

**Fixed:**
- Context counter showing 243/200K when real usage was 19K
- Streaming stop_reason overwritten by deferred usage delta

### v0.2.55 (2026-03-06)

**Added:**
- Cumulative usage ledger — Deleting sessions no longer resets usage stats

**Fixed:**
- Compaction overhaul — Zero-truncation, DB persistence, exhaustive summaries

### v0.2.54 (2026-03-05)

**Added:**
- Agent-to-Agent (A2A) protocol gateway
- Bee Colony structured debate
- Plans system with approval workflow

For older releases, see the [full changelog](https://github.com/adolfousier/opencrabs/blob/main/CHANGELOG.md).
