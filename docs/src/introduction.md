# Introduction

**OpenCrabs** is a self-hosted, provider-agnostic AI orchestration agent that runs as a single Rust binary. It automates your terminal, browser, channels (Telegram/Discord/Slack/WhatsApp/Trello), and codebase — all while respecting your privacy and keeping you in control.

## What Makes OpenCrabs Different

### 🔄 Provider-Agnostic with Native CLI Integration
- **13+ built-in providers**: Anthropic, OpenAI, Gemini, OpenRouter, Qwen (OAuth + CLI), MiniMax, Ollama, LM Studio, vLLM, NVIDIA, Dialagram, z.ai GLM, GitHub Copilot
- **Claude Code CLI & OpenCode CLI** integrated as native providers — use their models without API keys
- **Ollama as native provider** — run any local model via Ollama API without custom provider setup (v0.3.15)
- **Custom OpenAI-compatible backends** now stream thinking tokens, tool calls, and intermediate text exactly like native providers (v0.3.2)
- **Sticky fallback chain** — auto-failover to secondary providers on rate limits or errors
- **Prompt caching** across Anthropic, OpenRouter, Gemini, Qwen DashScope — reduces costs up to 95% (v0.3.2)

### 🤖 Multi-Agent Orchestration
- **Typed sub-agents**: `general`, `explore`, `plan`, `code`, `research` — each with tailored tool access
- **Team orchestration**: `team_create`, `team_broadcast`, `team_delete` for coordinated workflows
- **Spawn/wait/resume** sub-agents with A2A protocol support
- **ALWAYS_EXCLUDED tools** per agent type for safety boundaries

### 🌐 Channel-Native Communication
- **Telegram, Discord, Slack, WhatsApp, Trello** — respond to messages, send files, manage threads
- **Cross-channel crash recovery** — pending requests route back to originating channel on restart (v0.2.93)
- **DB-persisted channel sessions** — state survives restarts
- **Voice support** — local Whisper STT + Piper TTS, fully offline

### 🧠 Self-Healing & Self-Improvement (v0.3.7)
- **Recursive Self-Improvement (RSI)** — agent analyzes its own performance, identifies patterns, and autonomously rewrites brain files (v0.3.6)
- **Feedback ledger** — persistent SQLite table recording every tool success/failure, user correction, provider error (v0.3.6)
- **Phantom tool call detection** — catches when the model narrates file changes in prose without executing tools (v0.3.7)
- **Append-only brain files** — brain files (SOUL.md, TOOLS.md, etc.) are now append-only with backup-before-write to prevent data loss (v0.3.13)
- **Upstream template sync** — automatically syncs brain file templates from the repo with version gating and append-only diffs (v0.3.15)
- **RSI alert suppression** — suppresses alerts whose dimension already has a fix commit, stale alerts age out (v0.3.13)
- **Context budget management**: 65% soft / 90% hard compaction thresholds with 3-retry LLM fallback
- **Stuck stream detection**: 2048-byte rolling window catches repeating patterns, auto-recover
- **Gaslighting defense**: strips tool-refusal preambles mid-turn across 4+ phrase families
- **Auto-fallback on rate limits** — saves state mid-stream, resumes on fallback provider
- **Mid-stream decode retry** — 3x backoff before provider fallback (v0.3.0)
- **Non-streaming compatibility** — synthesizes full stream events from non-streaming JSON (v0.3.7)
- **Per-session message queue isolation** — prevents cross-session message bleeding in split panes and channels (v0.3.13)

### 🖥️ Terminal UI Excellence (v0.3.2)
- **Header card overlay** replaces splash screen — animated, responsive, vanishes after load
- **Select/Drag to Copy** — native mouse selection in TUI, auto-copies to clipboard on release
- **O(N) input render** — tall pastes no longer cause quadratic render cost; scroll-to-cursor preserved
- **Emoji cursor rendering** — grapheme cluster extraction for multi-byte emoji highlighting
- **Line navigation in multiline** — Up/Down navigates lines inside recalled multi-line input
- **F12 mouse capture toggle** — toggle native terminal text selection without exiting TUI
- **Async session load** — instant first paint, messages load in background

### 🔧 Developer Experience
- **Bang operator (`!cmd`)** — run shell commands directly from TUI input, no LLM round-trip (v0.3.1)
- **Full CLI surface**: 20+ subcommands (`/models`, `/approve`, `/compact`, `/rebuild`, `/evolve`, `/new`, `/doctor`, `/btw`, etc.)
- **`/btw` parallel agent** — spawn an isolated sub-agent for side tasks while the main conversation continues (v0.3.15)
- **Programmatic `/evolve`** — bypasses LLM, runs update directly (v0.3.1)
- **Auto-update on startup** — `[agent] auto_update = true` silently installs + hot-restarts (v0.3.1)
- **Dynamic tools** — runtime-defined via TOML (HTTP + shell executors)
- **Split panes** — tmux-style parallel sessions with layout persistence
- **Usage Dashboard** — `/usage` command shows daily tokens, cost, active models, session categories, project activity (v0.3.9)
- **Onboarding welcome** — personalized first-time detection with welcome message and guided setup (v0.3.13)
- **Recent file memory** — persists recent file paths across sessions to anchor the agent (v0.3.13)
- **Bash hardening** — rejects interactive commands up-front, short-circuits exact same failing command retries, tilde expansion fixed (v0.3.13)

### 🌐 Browser Automation
- **Full CDP support**: navigate, click, type, screenshot, JS eval, wait for selectors, find elements
- **`browser_find` tool** — enumerate elements by CSS, XPath, text, or aria-label with stable selectors (v0.3.13)
- **Headless or headed** mode, element-specific screenshots
- **Cookie/session persistence** across browser sessions
- **Per-session tab isolation** — no cross-session DOM stomping (v0.3.13)
- **Smart default browser detection** — auto-detects your default Chromium on macOS, Linux, and Windows (v0.3.13)

### 📊 Usage Analytics (v0.3.9)
- **Interactive dashboard** — `/usage` command with daily token counts, cost estimates, active models, session categories
- **Session auto-categorizer** — heuristic classification (dev, ops, research, chat, etc.)
- **Tool execution tracking** — DB records every tool call for per-project analytics
- **Project activity view** — normalized paths, category breakdown, token distribution
- **Soft-delete sessions** — metadata preserved even after session removal

### 🔐 Security & Privacy
- **Zero telemetry** — nothing sent anywhere, ever
- **API key security**: `zeroize` on drop, separate `keys.toml` (chmod 600)
- **Tool path resolution centralized** — tilde expansion, relative paths, symlink handling in one place (v0.3.2)
- **Auto-approve propagation** — `approval_policy = "auto-always"` actually reaches tool loop (v0.3.2)

### 📊 Testing & Quality
- **2,479+ tests** covering providers, tools, channels, TUI, self-healing, crash recovery, browser automation
- **6 new test categories**: subagent orchestration, team workflows, Telegram resume pipeline, token tracking, cross-channel recovery, cron execution storage
- **CI/CD**: GitHub Actions, CodeQL, release automation

## Quick Start

```bash
# Install (Linux/macOS)
ARCH=$(uname -m | sed 's/x86_64/amd64/;s/aarch64/arm64/')
OS=$(uname -s | tr A-Z a-z)
# Requires jq for reliable tag parsing; fallback to grep if unavailable
TAG=$(command -v jq >/dev/null 2>&1 && curl -s https://api.github.com/repos/adolfousier/opencrabs/releases/latest | jq -r .tag_name || curl -s https://api.github.com/repos/adolfousier/opencrabs/releases/latest | grep -o '"tag_name":"[^"]*"' | cut -d'"' -f4)
curl -fsSL "https://github.com/adolfousier/opencrabs/releases/download/${TAG}/opencrabs-${TAG}-${OS}-${ARCH}.tar.gz" | tar xz
./opencrabs

# Or via Cargo (requires Rust 1.94+)
cargo install opencrabs --locked

# Auto-update enabled by default; disable with [agent] auto_update = false in ~/.opencrabs/config.toml
```

## Architecture Overview

```
┌─────────────────────────────────────────┐
│           OpenCrabs Binary              │
│  (Single 17-22 MB Rust executable)      │
├─────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────────┐  │
│  │   TUI       │  │   CLI Daemon    │  │
│  │  (crossterm)│  │  (systemd/launchd)││
│  └─────────────┘  └─────────────────┘  │
│                                         │
│  ┌─────────────────────────────────┐   │
│  │        Provider Registry         │   │
│  │  • Native: Anthropic, OpenAI... │   │
│  │  • CLI: Claude Code, OpenCode   │   │
│  │  • Custom: any OpenAI-compatible│   │
│  │  • Fallback chain w/ sticky swap│   │
│  └─────────────────────────────────┘   │
│                                         │
│  ┌─────────────────────────────────┐   │
│  │        Tool Layer                │   │
│  │  • 50+ built-in tools           │   │
│  │  • Dynamic tools via TOML       │   │
│  │  • ALWAYS_EXCLUDED per agent    │   │
│  │  • Centralized path resolution  │   │
│  └─────────────────────────────────┘   │
│                                         │
│  ┌─────────────────────────────────┐   │
│  │        Channel Adapters          │   │
│  │  • Telegram/Discord/Slack/      │   │
│  │    WhatsApp/Trello/Voice        │   │
│  │  • Cross-channel crash recovery │   │
│  └─────────────────────────────────┘   │
│                                         │
│  ┌─────────────────────────────────┐   │
│  │        Self-Healing Layer       │   │
│  │  • Context budget management    │   │
│  │  • Stuck stream detection       │   │
│  │  • Gaslighting refusal strip    │   │
│  │  • Panic recovery + cancel persist││
│  └─────────────────────────────────┘   │
│                                         │
│  ┌─────────────────────────────────┐   │
│  │        Persistence              │   │
│  │  • SQLite sessions + memory DB  │   │
│  │  • Brain files (~/.opencrabs/)  │   │
│  │  • Hybrid FTS5 + vector search  │   │
│  └─────────────────────────────────┘   │
└─────────────────────────────────────────┘
```

## Next Steps

- [Getting Started](getting-started/installation.md) — Install and configure
- [Providers](brain/providers-overview.md) — Connect your LLM backends
- [Tools](brain/tools.md) — Explore 50+ built-in capabilities
- [Channels](channels/overview.md) — Connect Telegram, Discord, etc.
- [Self-Healing](features/self-healing.md) — Understand resilience features
- [Multi-Agent](features/multi-agent.md) — Orchestrate sub-agents and teams
