# OpenCrabs

**The autonomous AI agent. Single Rust binary. Every channel.**

OpenCrabs is an open-source AI agent that runs in your terminal. It connects to any LLM provider, orchestrates 50+ built-in tools, and works from Telegram, Discord, Slack, WhatsApp, or the built-in TUI. Typed sub-agents and team orchestration, multi-profile isolation, OpenRouter reasoning, cross-channel crash recovery, self-healing config, persistent split panes, browser automation, and 1,772 tests.

## Why OpenCrabs?

| | OpenCrabs | Node.js Frameworks |
|---|---|---|
| **Binary size** | 17-22 MB | 1 GB+ (node_modules) |
| **Runtime deps** | Zero | Node.js + npm + native modules |
| **API keys** | `keys.toml` (local file) | `.env` (process env pollution) |
| **Data residency** | Local SQLite | Varies |
| **Memory safety** | Rust (compile-time) | JavaScript (runtime) |

## Core Capabilities

- **Multi-provider AI** — Anthropic Claude, OpenAI, Google Gemini, OpenRouter (400+ models with reasoning support), MiniMax, z.ai GLM, Claude CLI, OpenCode CLI, Ollama, LM Studio, or any OpenAI-compatible API. Fallback chain for automatic failover. Per-session provider isolation. Function calling detection with model switch suggestions
- **Every channel** — Telegram, Discord, Slack, WhatsApp, or the built-in TUI with split panes. DB-persisted channel sessions survive restarts
- **50+ tools** — File ops, bash, web search, code execution, image gen, browser automation, and user-defined dynamic tools
- **Multi-agent & Teams** — Typed sub-agents (General, Explore, Plan, Code, Research) with filtered tool registries. Team orchestration: spawn N agents in parallel with `team_create`, fan-out messages with `team_broadcast`. Configurable subagent provider/model. Full CLI with 20+ subcommands
- **Persistent memory** — 3-tier memory system: daily notes, long-term memory, semantic search
- **Self-healing** — Auto-recovers corrupted config from last-known-good snapshots, tracks per-provider health with auto-failover, 65% context budget management with LLM compaction, stuck stream detection, emergency ARG_MAX compaction. Cross-channel crash recovery routes pending requests back to originating Telegram/Discord/Slack chat on restart. CLI idle timeout extended to 10 minutes (cargo builds don't time out). DB integrity checks. All recovery events delivered as visible notifications across TUI and channels
- **Self-evolving** — Type `/evolve` to download the latest version, or `/rebuild` to build from source. Startup update prompt offers one-click upgrades. `/evolve` works directly on channels without LLM routing
- **Shared channel commands** — `/doctor`, `/help`, `/usage`, `/evolve` execute instantly on all channels via a shared command handler (847-line module), no LLM round-trip required
- **Agent-to-Agent** — Built-in A2A gateway for peer-to-peer agent communication
- **Cron jobs** — Schedule isolated or main-session tasks with cron expressions
- **Browser automation** — Native headless Chrome control via CDP with smart browser detection
- **Voice** — Local STT (rwhisper/Metal GPU) and TTS (Piper), plus API options (Groq Whisper, OpenAI TTS)
- **Plans** — Structured multi-step task planning with approval workflow
- **Daemon mode** — Background operation with health endpoints and auto-reconnecting channel bots
- **Multi-profile** — Isolated instances per profile: separate config, brain files, sessions, and daemon services. Token-lock isolation, profile export/import as `.tar.gz`, migration between profiles
- **Cron job storage** — Execution results stored in DB for querying, multi-target delivery (comma-separated channels + webhooks)

## Quick Links

- [GitHub Repository](https://github.com/adolfousier/opencrabs)
- [Releases](https://github.com/adolfousier/opencrabs/releases)
- [Issues](https://github.com/adolfousier/opencrabs/issues)
- [Changelog](./changelog.md)
