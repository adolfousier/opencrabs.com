# Changelog

All notable changes to this project will be documented in this file.

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
