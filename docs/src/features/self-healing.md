# Self-Healing

OpenCrabs monitors its own health and automatically recovers from failures without user intervention. All recovery events surface as visible notifications across TUI and all channels.

## How It Differs from Crash Recovery

OpenCrabs has had **crash recovery** since early versions -- if the process dies mid-request, pending requests are tracked in SQLite and automatically resumed on restart (see [Pending Request Recovery](#pending-request-recovery) below).

**Self-healing** (v0.2.92) goes further: the agent detects and fixes problems *while it's still running* -- corrupted config, degraded providers, context overflow, stuck streams, DB corruption -- without restarting. Crash recovery is the safety net; self-healing prevents the fall.

## Config Recovery

Every successful write to `config.toml` creates a snapshot at `~/.opencrabs/config.last_good.toml`. When the config becomes corrupted or unparseable, OpenCrabs restores from the last-known-good snapshot automatically.

```
⚠️ Config was corrupted — restored from last-known-good snapshot (2 minutes ago)
```

A `CONFIG_RECOVERED` atomic flag tracks whether recovery happened during the current session, so downstream code can react accordingly.

### Unknown Key Detection

Unknown top-level keys in `config.toml` trigger a startup warning listing the unrecognized entries. This catches typos like `[teelgram]` or `[a2a_gatway]` before they cause silent misconfiguration.

Known valid sections: `[crabrace]`, `[database]`, `[logging]`, `[debug]`, `[providers]`, `[channels]`, `[agent]`, `[daemon]`, `[a2a]`, `[image]`, `[cron]`.

The `[a2a]` section also accepts `gateway` as an alias via serde, deduplicating a common typo.

### Custom Provider Name Normalization

Provider names with mixed case or whitespace (e.g. `"My Provider"` vs `"my provider"`) are normalized on load and save, preventing duplicate entries that would confuse the provider registry.

## Provider Health Tracking

Per-provider success/failure history is persisted to `~/.opencrabs/provider_health.json`. Each provider tracks:

- `last_success` and `last_failure` (epoch seconds)
- `last_error` (truncated to 200 chars)
- `consecutive_failures` count (resets on success)

```json
{
  "anthropic": {
    "last_success": 1743250500,
    "consecutive_failures": 0
  },
  "openai": {
    "last_success": 1743249800,
    "last_failure": 1743249700,
    "last_error": "rate_limit_exceeded",
    "consecutive_failures": 0
  }
}
```

The `/doctor` command surfaces health stats for every configured provider. Combined with the fallback provider chain, OpenCrabs detects degraded providers and routes to healthy ones automatically.

**Source:** `src/config/health.rs` (120 lines), integrated into `src/brain/agent/service/helpers.rs`.

## DB Integrity Check

SQLite `PRAGMA integrity_check` runs at startup. If corruption is detected, a notification appears in TUI and all connected channels instead of silently failing.

## Error Surfacing

v0.2.92 eliminated 14+ instances of silently swallowed errors across:

- Config writes
- Channel sends (Telegram, Discord, Slack, WhatsApp)
- Tool connections (Slack, WhatsApp, Trello connect tools)
- Pane state persistence

**Before:** `let _ = ...` and `.ok()` everywhere, errors vanish.
**After:** Every error surfaces via logging or user notification.

Onboarding config writes use `try_write!` macros that batch errors during wizard steps and report them all at the end, so users see exactly what failed.

### AgentService Config Propagation

`AgentService::new()` now requires an explicit `&Config` parameter instead of calling `Config::load()` internally. This eliminates hidden I/O, makes dependencies explicit, and enables test injection via `AgentService::new_for_test()`.

Render, dialogs, messaging, and cron modules no longer call `Config::load()` internally -- errors propagate up the call stack instead of being swallowed.

## Context Budget Management

The agent enforces a **65% context budget threshold**. When token usage reaches 65% of the effective context window (context limit minus tool schema overhead), automatic LLM compaction fires:

1. **Detect** context usage ≥ 65% of effective max tokens
2. **Compact** via LLM summarization (preserves meaning, not just truncation)
3. **Retry** up to 3 times if compaction fails
4. **Second pass** with tighter budget if still over threshold

The 65% threshold exists because providers like MiniMax degrade on function-calling quality well before hitting theoretical context limits -- tool calls break around ~133k tokens of a 200k limit.

**Source:** `src/brain/agent/service/tool_loop.rs` (lines 14-112)

## Emergency Compaction (ARG_MAX Recovery)

When CLI provider conversation context exceeds the OS `ARG_MAX` limit (~1MB on macOS), the agent recovers with a 3-stage fallback:

1. **Catch** the "Argument list too long" or "prompt too large" error
2. **Emergency compact** the conversation with an LLM summarization pass
3. **Insert** a system marker so the agent knows context was compacted
4. **Retry** the request

If compaction still fails, **hard truncation** kicks in -- keeps last 24 messages (12 conversation pairs) with a marker telling the agent to use `search_session` for older context. Both markers persist to DB for recovery across sessions.

Both actions emit `SelfHealingAlert` progress events so users see exactly what happened.

**Source:** `src/brain/agent/service/tool_loop.rs` (lines 550-687), tested with `ArgTooLongMockProvider` and `ContextLengthMockProvider` in `src/tests/cli_arg_too_long_test.rs` (352 lines).

## Stream Resilience

### Stuck Loop Detection

Some streaming providers (notably MiniMax) occasionally loop the same content indefinitely without sending a stop signal. The agent detects this:

- Maintains a **2048-byte rolling window** of recent streamed text
- When a **200+ byte substring** from the second half appears in the first half, it's a repeat
- Stream is terminated immediately and retry logic fires

**Source:** `src/brain/agent/service/helpers.rs` -- `detect_text_repetition()`, tested in `src/tests/stream_loop_test.rs` (15 tests)

### Idle Timeout

If a stream goes silent for **60 seconds** (API providers) or **10 minutes** (CLI providers) with no events, it's treated as a dropped connection.

CLI providers (Claude CLI, OpenCode CLI) run internal tools — cargo builds, tests, `gh` commands — that can take several minutes without producing stream events. The 60-second timeout caused premature termination on these, so CLI providers now get a 10-minute window before timeout fires.

If a stream goes silent:

```rust
const STREAM_IDLE_TIMEOUT: Duration = Duration::from_secs(60);
```

The `tokio::select!` loop races the stream against the timeout and the user's cancellation token. Timeout triggers retry, not a hard error.

## Pending Request Recovery

Crash recovery tracks every in-flight agent request in a `pending_requests` SQLite table. When a request starts, a row is inserted; when it completes (success or failure), the row is deleted.

On startup, any surviving rows mean the process crashed mid-request:

1. Query `pending_requests` for interrupted rows
2. Clear all rows (prevents double-recovery if this run also crashes)
3. Dedup by session_id (resume each session only once)
4. Spawn background tasks with a continuation prompt:
   > *"A restart just occurred while you were processing a request. Read the conversation context and continue where you left off naturally."*
5. Emit `TuiEvent::PendingResumed` so the TUI shows a recovery notification

**Source:** `src/db/repository/pending_request.rs`, `src/cli/ui.rs` (lines 705-790)

### Cross-Channel Crash Recovery (v0.2.93)

Before v0.2.93, pending request recovery always responded via the TUI — even if the original request came from Telegram, Discord, Slack, or WhatsApp. The resumed response would appear in the wrong place.

Now each channel passes its name and `chat_id` into `run_tool_loop`, which stores them in `pending_requests`. On restart, recovery routes responses back to the originating channel:

| Original channel | Recovery response goes to |
|---|---|
| Telegram | Same Telegram chat |
| Discord | Same Discord channel |
| Slack | Same Slack channel |
| WhatsApp | Same WhatsApp chat |
| Trello | Same Trello board |
| TUI | TUI (as before) |

The `pending_requests` table gained `channel` and `channel_chat_id` columns via a DB migration. `get_interrupted_for_channel` lets each channel handler query only its own pending rows. Selective `delete_ids` prevents one channel from clearing another channel's recovery entries.

## State Cleanup

Session deletion triggers **cascade deletes** across all related data:

- Messages (full conversation history)
- Usage ledger entries (token/cost records)
- Channel messages (Telegram, Discord, Slack, WhatsApp delivery records)
- Plans (autonomous plans created in the session)
- Cron jobs (scheduled tasks bound to the session)
- Cached pane state (stale split pane entries)

Custom provider names are **normalized** on load and save (`"My Provider"` → `"my-provider"`), preventing duplicate entries that would confuse the provider registry.

## Model Selector Safety

Pressing Enter in the model selector no longer clears existing API keys. The selector preserves current configuration while switching models.

Model switching errors now surface the actual error with a `⚠️` prefix on all channels, instead of always showing "Model switched" even on failure.

## UTF-8 Safety

`split_message()` across all 5 channel handlers (Telegram, Discord, Slack, WhatsApp, Trello) now uses `is_char_boundary()` to find safe split points, preventing panics on multi-byte characters (emojis, CJK, accented characters).

## Cancel Persistence (v0.2.97)

When a user double-Escapes to abort a streaming response, the partial content is now **persisted to the database before `handle.abort()` fires**. This means cancelled content survives a session reload -- you can scroll back and see exactly what the agent was saying before you stopped it.

### Claude CLI Subprocess Cleanup

Previously, aborting a Claude CLI request would orphan the underlying `claude` subprocess. Now the stream reader loop monitors `tx.closed()` via `tokio::select!` and kills the child process when the receiver drops, preventing leaked subprocesses accumulating in the background.

### Telegram Stale Delivery Suppression

When a request is cancelled mid-flight, the agent sometimes continued processing and delivered a stale response to Telegram. A `cancel_token.is_cancelled()` guard now fires before final delivery, preventing old agent results from posting after cancellation.

### Config Overwrite Protection

The onboarding wizard previously overwrote existing channel settings on every save, causing data loss when re-running `/onboard`. `apply_config()` now scopes writes to only the current onboarding step. `from_config()` sets `EXISTING_KEY_SENTINEL` for all existing channel data, ensuring untouched fields are never overwritten.

### Tool Description Wrapping

Tool call descriptions were previously truncated at 80 characters in the TUI. `render_tool_group` now **wraps** description headers and value lines to terminal width, and the 80-char pre-truncation of bash commands in `format_tool_description` has been removed. Long commands and file paths display fully.

## Notifications

All self-healing events are delivered to:
- TUI (status bar notification)
- Telegram, Discord, Slack, WhatsApp (if connected)

Nothing happens silently. If the crab fixes itself, it tells you what it fixed.
