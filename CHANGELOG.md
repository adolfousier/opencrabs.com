# Changelog

All notable changes to this project will be documented in this file.

## [0.1.16] - 2026-05-21

### Added
- RTK Token Savings integration — bundled RTK binary (4MB, v0.40.0) as default feature, zero-config proxy, 100+ commands, `/rtk` slash command (PR #102)
- Tool call stacking — 3+ consecutive tool call groups collapse into single summary line, Ctrl+O toggle (5 commits, 6 tests)
- `hashline_edit` tool — hash-anchored file editing with 2-char content hashes, stale hash rejection, batch edits (#60)
- Sensitive data redaction — applied to TUI and all channels, new patterns (env var suffixes, piped secrets)
- Context budget footer for channels — "ctx: 8K/200K 4%" on Telegram, Discord, Slack, WhatsApp (#104)
- Brain file cleanup_intent — user-driven maintenance flag, RSI blocked from shrinking (#103)

### Changed
- Test count updated: 2,834 → 2,975
- Auto-title fires at end of first turn across all channels
- Sessions sort by last interaction time

### Fixed
- Compaction dropped 55% kept-tail — summary IS the conversation
- Self-heal 5-nudge budget for reasoning-only turns + sticky fallback
- Completion-escape clause for phantom enforcement messages
- Scroll fixes — removed load_more_history overshoot, preserved scroll during streaming
- WhatsApp photo batching for multi-image uploads
- Telegram media_group_id-based photo batching
- Gemini schema strips default/example from tool schemas (#101, @leshchenko1979)
- Custom provider model selection persistence
- Compaction prompt dominance + plan tool descriptions
- Tool loop borrow-after-move fix from PR #100 merge

## [0.1.11] - 2026-05-18

### Added
- By-Model Quantization Tree View — usage dashboard groups model variants (qwen3.6-35b-a3b-gguf, -oq2, -oq4) under parent row with tree connectors (├─/└─), aggregated tokens/cost/calls
- Per-pane error & notification banners — new TUI feature for error visibility per pane
- RSI home directory resolution fix — resolves ~ to actual home dir instead of CWD-relative
- Bare tool-call array catch — top-level arrays from models no longer leak into TUI or crash RSI
- Load tools.toml in run/agent modes — dynamic tools now work outside TUI (external contribution, #79, @leshchenko)

### Changed
- Provider table: 13 → 15 entries (added OpenCode + Codex CLI, renamed Codex OAuth → Codex)
- Provider count: 12+ → 15
- Test count updated: 2,698 → 2,737

### Fixed
- Stuck intent loop abort — infinite retry replaced with clear abort notification
- Phantom-exhaustion text replaced with abort notice
- Cron BLOB recovery — tolerates BLOB-typed prompt rows
- Slash/emoji popup height clamped to terminal height
- LICENSE reference path fix (PR #82, @kriptoburak)

## [0.1.10] - 2026-05-15

### Added
- Codex OAuth provider documentation — native device-code PKCE flow, no CLI dependency
- OpenAI-compatible embedding API docs — external providers (OpenAI, Ollama, Jina, LM Studio)
- FTS5-only memory mode for VPS — zero RAM overhead keyword search
- Three embedding modes documented in memory.md
- RSI feedback fix documentation — records actual model used after remap
- Tool loop reasoning markers persistence docs
- @ file picker large repo fix docs
- Cron provider isolation and mismatched pair validation docs
- Windows CI and Node 24 fix docs
- Codex OAuth device flow field name fixes docs

### Changed
- Test count updated: 2,595 → 2,698
- Provider count updated: 11+ → 12+ (added Codex OAuth)
- Introduction synced with v0.3.18 + v0.3.19
- Providers overview: added Codex OAuth row
- Providers setup: added Codex OAuth section with PKCE flow details
- Self-Improvement: added v0.3.19 additions section
- Self-Healing: added v0.3.19 fixes section
- Memory system: added Embedding Modes section (Local GGUF, API, FTS5-only)

## [0.1.9] - 2026-04-14

### Added
- Usage Dashboard — interactive TUI overlay showing daily tokens, cost, active models, session categories, and project activity
- Session auto-categorizer with heuristic classification (dev, ops, research, etc.)
- Tool execution tracking in DB for usage analytics
- Soft-delete sessions to preserve metadata for usage stats
- Project/activity data with normalized paths and category support
- Dashboard UI polish: reverse daily order, centered header/footer, dynamic column widths
- **1,827 → 1,995 tests** (+168: Usage module, A2A debate, Dynamic Tools, Session categorizer)

### Changed
- Synced docs with OpenCrabs v0.3.9

## [0.1.8] - 2026-04-13

### Added
- Self-Improvement (RSI) documentation — feedback ledger, pattern analysis, autonomous brain file updates
- Qwen multi-account rotation docs — 2-10 accounts with auto-advance on 429
- Non-streaming compatibility module docs
- Phantom tool call detection docs
- Updated architecture diagram with RSI layer

### Updated
- Introduction: Self-Healing section → Self-Healing & Self-Improvement (v0.3.7)
- Added RSI, feedback ledger, phantom detection, non-streaming compat, mid-stream decode retry
- SUMMARY.md: added Self-Improvement (RSI) page entry

## [0.1.7] - 2026-04-07

### Added
- Bang operator (`!cmd`) documentation — run shell commands directly from TUI without LLM round-trip
- Auto-update on startup (`[agent] auto_update` config flag, default `true`)
- Programmatic `/evolve` — runs directly, no longer routed through LLM
- F12 mouse capture toggle for native terminal text selection
- Landing page: anschmieg review ("OpenCrabs is the first agent I click with")
- CLI commands doc: bang operator, auto-update config, keyboard shortcuts section

### Changed
- Test count updated to 1,827+
- Introduction updated with bang operator, auto-update on startup

## [0.1.6] - 2026-04-07

### Added
- Qwen Code CLI provider added to providers overview (1k free req/day via OAuth, qwen3-coder-plus, qwen3.5-plus, qwen3.6-plus)
- Self-healing: auto-fallback on rate limits (saves state, resumes on fallback provider), two-tier context budget enforcement (65% soft / 90% hard), mid-stream decode retry (3x backoff), SIGINT handler + panic hook, proactive rate limiting for OpenRouter :free models
- Introduction: vision-first file processing, Qwen Code CLI, auto-fallback on rate limits
- Testing: 1,772 → 1,827 tests (+55 Telegram resume pipeline tests)
- Landing page: updated hero with 1,827 tests, 11+ providers

### Fixed
- **Issue #56**: Installation download URL broken on Ubuntu 24 — `uname -m` produced `x86_64` but release assets use `amd64`. Fixed with proper arch mapping (`x86_64→amd64`, `aarch64→arm64`) and version-tagged URLs in both docs and landing page

### Changed
- Synced docs with OpenCrabs v0.2.98, v0.2.99, v0.3.0

## [0.1.5] - 2026-04-04

### Added
- New docs: Multi-Agent Orchestration (typed agents, team orchestration, subagent provider config)
- Multi-agent: Agent type system (General, Explore, Plan, Code, Research) with filtered tool registries and ALWAYS_EXCLUDED enforcement
- Multi-agent: Team orchestration (team_create, team_delete, team_broadcast) for parallel agent groups
- Multi-agent: Subagent provider/model config ([agent] section in config.toml)
- Multi-agent: send_input loop for multi-turn child agent conversations
- Self-healing: cancel persistence (streaming state saved to DB before abort)
- Self-healing: Claude CLI subprocess cleanup on cancel (tokio::select! on tx.closed())
- Self-healing: Telegram stale delivery suppression (cancel_token guard before final delivery)
- Self-healing: config overwrite protection in onboarding (scoped writes, EXISTING_KEY_SENTINEL)
- Self-healing: tool description wrapping (no more 80-char truncation of bash commands)
- SUMMARY: Multi-Agent Orchestration entry added to Features section
- Landing page: Multi-Agent & Teams feature card, updated hero with 1,772 tests
- Tools: team_create, team_delete, team_broadcast added to agent orchestration section; agent types table
- Testing: 1,687 → 1,772 tests (+84 subagent/team tests covering manager state machine, SendInput, CloseAgent, WaitAgent, lifecycle, AgentType filtering, TeamManager, TeamDelete, TeamBroadcast, registry exclusion)

### Changed
- Synced docs with OpenCrabs v0.2.97

## [0.1.4] - 2026-04-03

### Added
- New docs: Multi-Profile support (create/migrate/export/import, token-lock isolation, profile-aware daemons, per-session provider isolation)
- Self-healing: cross-channel crash recovery routing (v0.2.93) — pending requests now route back to originating Telegram/Discord/Slack/WhatsApp chat on restart; CLI idle timeout extended to 10 minutes
- Cron jobs: multi-target delivery (comma-separated targets), execution results stored in DB
- Providers: OpenRouter reasoning support (include_reasoning, collapsible thinking sections), function calling detection, tool_choice: "auto" for OpenAI-compatible providers, per-session provider isolation
- SUMMARY: Multi-Profile entry added to Features section
- Landing page: Multi-Profile feature card, updated hero with 1,687 tests
- Testing: 1,593 → 1,687 tests with profile tests (57), token tracking tests (29), cross-channel crash recovery tests, LLM artifact stripping tests, cron results tests

### Changed
- Synced docs with OpenCrabs v0.2.93, v0.2.94, v0.2.95, v0.2.96

## [0.1.3] - 2026-03-30

### Added
- Self-Healing docs: comprehensive page covering crash recovery vs self-healing distinction, config recovery with last-known-good snapshots, provider health tracking with auto-failover, 65% context budget management with 3-retry LLM compaction, stuck stream detection (2048-byte rolling window, 200-byte match threshold), 60s stream idle timeout, emergency ARG_MAX compaction (3-stage fallback), pending request recovery (crash replay from SQLite), cascade state cleanup on session delete, unknown config key detection, custom provider name normalization, model selector safety, UTF-8 split safety, DB integrity checks
- Architecture: shared channel commands module (`commands.rs`, 847 lines), config health tracking module (`health.rs`, 120 lines), expanded data flow with stream resilience and crash recovery
- CLI commands: shared command handler details, `/evolve` direct channel execution
- Sessions: state management improvements (reload after cancel, cache cleanup, CLI tool segment persistence, case-insensitive tool input)
- Split panes: state management fixes (tool spinner, scroll position, stale cache cleanup)
- Tools: error handling section for tool connection surfacing
- Landing page: updated Self-Healing card with context budget, crash recovery, stream resilience, 14+ error surfacing
- Introduction: expanded self-healing with context budget, stream detection, crash recovery, error surfacing counts

### Changed
- Hero description: added provider health tracking
- Architecture diagram: added shared channel commands layer, expanded self-healing with ARG_MAX compaction and error surfacing
- Architecture data flow: 6 steps to 8 (added channel commands, self-healing monitoring)
- Synced docs with OpenCrabs v0.2.91 and v0.2.92

## [0.1.2] - 2026-03-28

### Added
- New docs: Split Panes, Dynamic Tools, Browser Automation
- Landing page: Split Panes, Multi-Agent, Browser Automation feature cards
- Providers: added Claude CLI, OpenCode CLI, MiniMax, Browser (CDP) badges

### Changed
- Tool count: 45+ to 50+ across landing page and docs
- Test count: 1,424 to 1,562 in testing docs
- Rust version: 1.93 to 1.94 in building docs
- Cargo/Source install: "Rust nightly" to "stable Rust"
- CLI commands page: expanded from 3 to 15+ subcommands (agent, service, db, logs, memory, session, etc.)
- Architecture diagram: added split panes, dynamic tools, browser, sub-agents, daemon
- Introduction: updated capabilities list with multi-agent, browser, daemon mode
- Sessions doc: added channel sessions and split pane references
- Tools doc: added browser automation and dynamic tools sections
- Building doc: added `browser` feature flag
- Synced docs with OpenCrabs v0.2.86 through v0.2.90

## [0.1.0] - 2026-03-06

### Added
- Initial release
- Leptos 0.8 WASM landing page
- Docker + Nginx deployment setup
- Screenshot assets

## [0.1.15] - 2026-05-20 (OpenCrabs v0.3.24 sync)

### Added
- `rename_session` tool — agent-driven session title renaming
- `follow_up_question` tool — multi-choice questions via channel buttons (Telegram inline, Discord components, Slack Block Kit, WhatsApp quick replies) (#94)
- Auto-generated session titles from first user message (background LLM call)
- Dynamic tools per-parameter `coerce_empty_to` / `coerce_null_to` for graceful empty/null handling (#95)
- Custom provider UX: paste-by-default on API key input, Enter-to-load typed models, field refresh after save
- Tool System catalog updated with `rename_session`, `follow_up_question`, and other long-missing built-ins

### Fixed
- Gemini provider strips `additionalProperties` from tool schemas before send (#99)
- Per-session model override now surfaces in TUI display (not just on the wire) (#97)
- CLI subcommands (`doctor`, `init`) use correct `--config`/`--keys` paths instead of hardcoded defaults (#96)
- `keys_path` type corrected (`PathBuf` → `Option<PathBuf>`)

### Changed
- Test count: 2,796 → 2,834
- Custom provider dialog accepts typed-not-in-list models and merges with fetched list
