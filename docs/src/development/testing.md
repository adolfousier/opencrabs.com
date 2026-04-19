# Testing Guide

Comprehensive test coverage for OpenCrabs. All tests run with:

```bash
cargo test --all-features
```

## Quick Reference

| Category | Tests | Location |
|----------|------:|----------|
| **Tests — CLI Parsing** | 28 | `src/tests/cli_test.rs` |
| **Tests — Cron Jobs & Scheduling** | 49 | `src/tests/cron_test.rs` |
| **Tests — Channel Search** | 24 | `src/tests/channel_search_test.rs` |
| **Tests — Voice STT Dispatch** | 11 | `src/tests/voice_stt_dispatch_test.rs` |
| **Tests — Voice Onboarding** | 62 | `src/tests/voice_onboarding_test.rs` |
| **Tests — Candle Whisper** | 6 | `src/tests/candle_whisper_test.rs` |
| **Tests — Evolve (Self-Update)** | 23 | `src/tests/evolve_test.rs` |
| **Tests — Session & Working Dir** | 15 | `src/tests/session_working_dir_test.rs` |
| **Tests — Message Compaction** | 24 | `src/tests/compaction_test.rs` |
| **Tests — Fallback Vision** | 35 | `src/tests/fallback_vision_test.rs` |
| **Tests — GitHub Copilot Provider** | 38 | `src/tests/github_provider_test.rs` |
| **Tests — File Extract** | 36 | `src/tests/file_extract_test.rs` |
| **Tests — Image Utils** | 9 | `src/tests/image_util_test.rs` |
| **Tests — Onboarding Brain** | 21 | `src/tests/onboarding_brain_test.rs` |
| **Tests — Onboarding Navigation** | 26 | `src/tests/onboarding_navigation_test.rs` |
| **Tests — Onboarding Types** | 16 | `src/tests/onboarding_types_test.rs` |
| **Tests — Onboarding Keys** | 4 | `src/tests/onboarding_keys_test.rs` |
| **Tests — OpenAI Provider** | 16 | `src/tests/openai_provider_test.rs` |
| **Tests — Plan Document** | 15 | `src/tests/plan_document_test.rs` |
| **Tests — TUI Error** | 16 | `src/tests/tui_error_test.rs` |
| **Tests — Queued Messages** | 15 | `src/tests/queued_message_test.rs` |
| **Tests — Custom Provider** | 27 | `src/tests/custom_provider_test.rs` |
| **Tests — Context Window** | 14 | `src/tests/context_window_test.rs` |
| **Tests — Onboarding Field Nav** | 46 | `src/tests/onboarding_field_nav_test.rs` |
| **Tests — Provider Sync** | 10 | `src/tests/provider_sync_test.rs` |
| **Tests — Brain Templates** | 8 | `src/tests/brain_templates_test.rs` |
| **Tests — Collapse Build Output** | 9 | `src/tests/collapse_build_output_test.rs` |
| **Tests — Reasoning Lines** | 6 | `src/tests/reasoning_lines_test.rs` |
| **Tests — AltGr Input** | 8 | `src/tests/altgr_input_test.rs` |
| **Tests — System Continuation** | 6 | `src/tests/system_continuation_test.rs` |
| **Tests — QR Render** | 8 | `src/tests/qr_render_test.rs` |
| **Tests — WhatsApp State** | 7 | `src/tests/whatsapp_state_test.rs` |
| **Tests — Post-Evolve** | 5 | `src/tests/post_evolve_test.rs` |
| **Tests — Stream Loop Detection** | 15 | `src/tests/stream_loop_test.rs` |
| **Tests — XML Tool Fallback** | 10 | `src/tests/xml_tool_fallback_test.rs` |
| **Tests — TUI Render Clear** | 4 | `src/tests/tui_render_clear_test.rs` |
| **Tests — Split Panes** | 21 | `src/tests/split_pane_test.rs` |
| **Tests — Slack Formatting** | 21 | `src/tests/slack_formatting_test.rs` |
| **Tests — Daemon Health** | 10 | `src/tests/daemon_health_test.rs` |
| **Tests — Claude CLI Cache** | 5 | `src/tests/claude_cli_cache_test.rs` |
| **Tests — Browser Headless** | 4 | `src/tests/browser_headless_test.rs` |
| **Tests — Provider Registry** | 8 | `src/tests/provider_registry_test.rs` |
| **Tests — Self-Healing System** | 27 | `src/tests/self_healing_test.rs` |
| **Tests — Emergency Compaction** | 2 | `src/tests/compaction_test.rs` |
| **Tests — Cross-Channel Crash Recovery** | 12 | `src/tests/pending_request_test.rs` |
| **Tests — Profile System** | 57 | `src/tests/profile_test.rs` |
| **Tests — Token Tracking** | 29 | `src/tests/token_tracking_test.rs` |
| **Tests — Cron Execution Storage** | 6 | `src/tests/cron_results_test.rs` |
| **Tests — LLM Artifact Stripping** | 8 | `src/tests/artifact_strip_test.rs` |
| **Tests — Subagent & Team Orchestration** | 84 | `src/tests/subagent_test.rs` |
| **Tests — Telegram Resume Pipeline** | 55 | `src/tests/telegram_resume_test.rs` |
| **Brain (all modules)** | 365 | `src/brain/` |
| **TUI (all modules)** | 141 | `src/tui/` |
| **Channels (all modules)** | 105 | `src/channels/` |
| **Utils (all modules)** | 56 | `src/utils/` |
| **Services (all modules)** | 44 | `src/services/` |
| **DB (all modules)** | 40 | `src/db/` |
| **Config (all modules)** | 32 | `src/config/` |
| **A2A (all modules)** | 21 | `src/a2a/` |
| **Usage** | 17 | `src/usage.rs` |
| **Pricing** | 17 | `src/pricing.rs` |
| **Memory** | 9 | `src/memory/` |
| **Logging** | 4 | `src/logging/` |
| **Total** | **2,056** | |

---

## Feature-Gated Tests

Some tests only compile/run with specific feature flags:

| Feature | Tests |
|---------|-------|
| `local-stt` | Local whisper inline tests, candle whisper tests, STT dispatch local-mode tests, codec tests, availability cycling tests |
| `local-tts` | TTS voice cycling, Piper voice Up/Down |

All feature-gated tests use `#[cfg(feature = "...")]` and are automatically included when running with `--all-features`.

---

## Running Tests

```bash
# Run all tests (recommended)
cargo test --all-features

# Run a specific test module
cargo test --all-features -- voice_onboarding_test

# Run a single test
cargo test --all-features -- is_newer_major_bump

# Run with output (for debugging)
cargo test --all-features -- --nocapture

# Run only local-stt tests
cargo test --features local-stt -- local_whisper
```

---

## Disabled Test Modules

These modules exist but are commented out in `src/tests/mod.rs` (require network or external services):

| Module | Reason |
|--------|--------|
| `error_scenarios_test` | Requires mock API server |
| `integration_test` | End-to-end with LLM provider |
| `plan_mode_integration_test` | End-to-end plan workflow |
| `streaming_test` | Requires streaming API endpoint |
