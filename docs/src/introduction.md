# Introduction

**OpenCrabs** is a self-hosted, provider-agnostic AI orchestration agent that runs as a single Rust binary. It automates your terminal, browser, channels (Telegram/Discord/Slack/WhatsApp/Trello), and codebase, all while respecting your privacy and keeping you in control.

**4,350+ tests** across providers, tools, channels, TUI, self-healing, and browser automation.

## What Makes OpenCrabs Different

### Provider-Agnostic by Design
- **15 built-in providers + Custom OpenAI Compatible**: Anthropic Claude, OpenAI, Gemini, Xiaomi MiMo, OpenRouter, Qwen (DashScope), MiniMax, Ollama, z.ai GLM, GitHub Copilot, Codex, Codex CLI, OpenCode, OpenCode CLI
- **Native CLI integration** — use Claude Code CLI, OpenCode CLI, and Codex CLI as providers without API keys
- **Sticky fallback chain** — auto-failover on rate limits or errors, health-aware persistence survives restarts
- **Prompt caching** across Anthropic, OpenRouter, Gemini, Qwen DashScope reduces costs up to 95%
- **Context window override** — cap or expand context for any model via config
- **Xiaomi MiMo** — 30+ models including the MiMo reasoning series, keyed provider

### Multi-Agent Orchestration
- **Sessions are fully isolated agents** — each with its own brain, provider, model, working directory, and history
- **Typed sub-agents**: `general`, `explore`, `plan`, `code`, `research` with tailored tool access
- **Team orchestration**: `team_create`, `team_broadcast`, `team_delete` for coordinated workflows
- **Per-call provider/model overrides** — mix models across teams (plan with GLM, code with Deepseek, review with Kimi)
- **A2A protocol** — JSON-RPC 2.0 gateway for agent-to-agent communication

### Channel-Native Communication
- **Telegram, Discord, Slack, WhatsApp, Trello** — full bot integration with DMs, groups, and threads
- **Telegram rich messages** — native tables, headings, lists, math via `rich_messages` config
- **Draft message streaming** — live "typing..." updates as tokens generate in DMs
- **Collapsible blocks** — `<details>/<summary>` sections for long outputs
- **Forum topic session isolation** — each topic in Telegram supergroups gets its own session
- **`/goal` across all channels** — set autonomous goals from Telegram, Discord, Slack, WhatsApp
- **Voice support** — local Whisper STT + Piper TTS, fully offline
- **Cross-channel crash recovery** — pending requests route back to originating channel on restart
- **`/cowork`** — create shared workspaces from channels and TUI
- **`/rename`, `/profiles`, `/cd`** — manage sessions, profiles, and directories from any channel

### Self-Healing & Self-Improvement
- **Recursive Self-Improvement (RSI)** — agent analyzes performance, identifies patterns, and rewrites brain files
- **Phantom tool call detection** — catches when the model narrates changes without executing tools
- **System brain rebuild** — brain files rebuilt from disk when changed, no restart needed
- **Proactive tool discovery** — searches for available tools before claiming inability
- **JIT tool activation** — extended tools activated on-demand, no pre-registration needed
- **Config auto-repair** — auto-repair broken config.toml, never poison last-good config
- **Context budget management** — 65% soft / 90% hard compaction thresholds with LLM fallback
- **Stuck stream detection** — 2048-byte rolling window catches repeating patterns
- **Gaslighting defense** — strips tool-refusal preambles mid-turn

### Terminal UI
- **Native markdown rendering** — emphasis, lists, links, and task items render directly in the terminal
- **Real-time tok/s throughput meter** — live tokens-per-second during streaming
- **Session search** — search filter + viewport scroll across all sessions
- **Split panes** — tmux-style parallel sessions with layout persistence
- **Clipboard image paste** — paste images from browser or any app directly into TUI
- **Plan pinning** — active plan pinned at end of prompt each turn
- **Agent-driven onboarding** — personalized first-time setup with guided flow
- **`/goal` autonomous loop** — set a goal and the agent loops until an LLM judge says it's done, with pause/resume/status controls

### Developer Experience
- **50+ built-in tools** — file ops, bash, web search, code execution, browser automation, image gen, voice, PDF rendering
- **Proactive tool discovery** — agent finds tools before saying "I can't"
- **Skills system** — workflow templates with fuzzy-finding, auto-registered as slash commands
- **Dynamic tools** — runtime-defined via TOML (HTTP + shell executors)
- **Projects system** — dedicated sessions with per-project brain overlays, file archiving, and color badges
- **Hashline editing** — hash-anchored file editing with batch support and collision detection
- **Mission Control** — full-screen dashboard with RSI inbox, activity log, and cron schedule
- **RTK auto-download** — bundled 4MB proxy for 53.5% token savings on 100+ commands
- **Confidential file protection** — SSH keys, .env, credentials protected by default
- **AGENTS always-loaded** — hard rules and governance enforced every turn

### Browser Automation
- **Full CDP support** — navigate, click, type, screenshot, JS eval, find elements
- **Headless or headed** mode with element-specific screenshots
- **Cookie/session persistence** across browser sessions
- **Per-session tab isolation** — no cross-session DOM stomping

## Quick Start

```bash
# Install (Linux/macOS)
ARCH=$(uname -m | sed 's/x86_64/amd64/;s/aarch64/arm64/')
OS=$(uname -s | tr A-Z a-z)
TAG=$(command -v jq >/dev/null 2>&1 && curl -s https://api.github.com/repos/adolfousier/opencrabs/releases/latest | jq -r .tag_name || curl -s https://api.github.com/repos/adolfousier/opencrabs/releases/latest | grep -o '"tag_name":"[^"]*"' | cut -d'"' -f4)
curl -fsSL "https://github.com/adolfousier/opencrabs/releases/download/${TAG}/opencrabs-${TAG}-${OS}-${ARCH}.tar.gz" | tar xz
./opencrabs

# Or via Cargo (requires Rust 1.94+)
cargo install opencrabs --locked
```

Auto-update enabled by default. Disable with `[agent] auto_update = false` in `~/.opencrabs/config.toml`.

## Architecture

```
┌─────────────────────────────────────────┐
│           OpenCrabs Binary              │
│  (Single 34-36 MB Rust executable)      │
├─────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────────┐  │
│  │   TUI       │  │   CLI Daemon    │  │
│  │  (crossterm)│  │  (systemd/launchd)││
│  └─────────────┘  └─────────────────┘  │
│                                         │
│  ┌─────────────────────────────────┐   │
│  │        Provider Registry         │   │
│  │  15 built-in + Custom OpenAI    │   │
│  │  Sticky fallback chain          │   │
│  └─────────────────────────────────┘   │
│                                         │
│  ┌─────────────────────────────────┐   │
│  │        Tool Layer                │   │
│  │  50+ built-in tools             │   │
│  │  Dynamic tools via TOML         │   │
│  │  JIT activation                 │   │
│  └─────────────────────────────────┘   │
│                                         │
│  ┌─────────────────────────────────┐   │
│  │        Channel Adapters          │   │
│  │  Telegram / Discord / Slack /    │   │
│  │  WhatsApp / Trello / Voice      │   │
│  └────────────────���────────────────┘   │
│                                         │
│  ┌─────────────────────────────────┐   │
│  │        Self-Healing Layer       │   │
│  │  Context budget / Stuck stream  │   │
│  │  Phantom detection / RSI        │   │
│  └─────────────────────────────────┘   │
│                                         │
│  ┌─────────────────────────────────┐   │
│  │        Persistence              │   │
│  │  SQLite + Brain files           │   │
│  │  FTS5 + vector search           │   │
│  └─────────────────────────────────┘   │
└─────────────────────────────────────────┘
```

## Next Steps

- [Installation](getting-started/installation.md) — Install and configure
- [Configuration](getting-started/configuration.md) — All config options
- [Providers](brain/providers-overview.md) — Connect your LLM backends
- [Channels](channels/overview.md) — Connect Telegram, Discord, etc.
- [Tools](brain/tools.md) — Explore 50+ built-in capabilities
- [Self-Healing](features/self-healing.md) — Resilience features
- [Multi-Agent](features/multi-agent.md) — Orchestrate sub-agents and teams
