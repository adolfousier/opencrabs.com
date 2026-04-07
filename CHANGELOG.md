# Changelog

All notable changes to this project will be documented in this file.

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
