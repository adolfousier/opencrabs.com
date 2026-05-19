# Introduction

**OpenCrabs** is a self-hosted, provider-agnostic AI orchestration agent that runs as a single Rust binary. It automates your terminal, browser, channels (Telegram/Discord/Slack/WhatsApp/Trello), and codebase — all while respecting your privacy and keeping you in control.

## What Makes OpenCrabs Different

### 🔄 Provider-Agnostic with Native CLI Integration
- **14 built-in providers + Custom OpenAI Compatible**: Anthropic Claude, OpenAI, Gemini, OpenRouter, Qwen (DashScope), MiniMax, Ollama, z.ai GLM, GitHub Copilot, Codex, Codex CLI, OpenCode, OpenCode CLI
- **Claude Code CLI, OpenCode CLI, Codex CLI & Codex OAuth** integrated as native providers — use their models without API keys
- **Codex (OAuth)** — native OpenAI Codex subscription auth via device-code PKCE flow. No CLI dependency, no API key. Authenticate through browser once; tokens stored with automatic refresh (v0.3.19)
- **Ollama as native provider** — run any local model via Ollama API without custom provider setup (v0.3.15)
- **Custom OpenAI-compatible backends** now stream thinking tokens, tool calls, and intermediate text exactly like native providers (v0.3.2)
- **Sticky fallback chain** — auto-failover to secondary providers on rate limits or errors
- **Health-aware sticky fallback persistence** — fallback state survives restarts, checks provider health on creation and advances if primary has 2+ consecutive failures (v0.3.17)
- **OpenRouter response caching** — zero cost for identical requests (v0.3.17)
- **TCP keepalive on all HTTP clients** — 15s keepalive detects silent TCP drops in ~15-45s instead of waiting for 300s idle timeout (v0.3.17)
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
- **Video uploads across all channels** — Slack, Telegram, Discord, WhatsApp, and Trello automatically route video attachments to `analyze_video` when vision is enabled (v0.3.17)
- **Telegram 20 MB Bot API cap** — surfaces "compress to under 20 MB and resend" message instead of silently failing on large uploads (v0.3.17)
- **Telegram dropped video/animation** — now downloads and routes through vision processing, including iPhone `.mov` uploads auto-converted to MP4-backed Animation (v0.3.17)
- **Slack intermediate-vs-final dedup race closed** — captures and awaits all IntermediateText JoinHandles before dedup check, with post-completion sweep for late entries (v0.3.17)
- **Clean display text** — all channels persist clean text to DB and TUI instead of LLM metadata brackets (v0.3.17)

### 🧠 Self-Healing & Self-Improvement (v0.3.7)
- **Recursive Self-Improvement (RSI)** — agent analyzes its own performance, identifies patterns, and autonomously rewrites brain files (v0.3.6)
- **Feedback ledger** — persistent SQLite table recording every tool success/failure, user correction, provider error (v0.3.6)
- **Phantom tool call detection** — catches when the model narrates file changes in prose without executing tools (v0.3.7)
- **Expanded phantom detection** — catches "Now \<file-op gerund>" phantoms (creating/writing/editing) and build/deploy intent + past-tense completion claims. Gaslighting and phantom detectors extracted into their own module (v0.3.17)
- **RSI escalation for repeat violations** — violation counter bumps on existing rules instead of deduping away. Rules that keep getting broken get louder, not silenced (v0.3.17)
- **RSI feedback records actual model used** — when helpers remap a mismatched model to the provider default, RSI now records the resolved model instead of the impossible original pair (v0.3.19)
- **Append-only brain files** — brain files (SOUL.md, TOOLS.md, etc.) are now append-only with backup-before-write to prevent data loss (v0.3.13)
- **Upstream template sync** — automatically syncs brain file templates from the repo with version gating and append-only diffs (v0.3.15)
- **RSI alert suppression** — suppresses alerts whose dimension already has a fix commit, stale alerts age out (v0.3.13)
- **Partial JSON repair** — closes unterminated strings, balances brackets, strips trailing commas. Wired into 5 drop sites across OpenAI-compatible providers (v0.3.17)
- **Context budget management**: 65% soft / 90% hard compaction thresholds with 3-retry LLM fallback
- **Stuck stream detection**: 2048-byte rolling window catches repeating patterns, auto-recover
- **Gaslighting defense**: strips tool-refusal preambles mid-turn across 4+ phrase families
- **Auto-fallback on rate limits** — saves state mid-stream, resumes on fallback provider
- **RetryAttempt progress event** — TUI shows "⏳ Retry 2/3 — stream dropped" so you see transient recovery in progress (v0.3.17)
- **Mid-stream decode retry** — 3x backoff before provider fallback (v0.3.0)
- **Non-streaming compatibility** — synthesizes full stream events from non-streaming JSON (v0.3.7)
- **Per-session message queue isolation** — prevents cross-session message bleeding in split panes and channels (v0.3.13)
- **Tool loop reasoning markers persisted** — reasoning content survives across tool loop iterations (v0.3.19)
- **@ file picker fixed for large repos** — skips .git/.hg/.svn directories, raised result cap to 20k (v0.3.19)

### 🖥️ Terminal UI Excellence (v0.3.2)
- **Per-pane error & notification banners** — dedicated error/notification display per TUI pane for better visibility (v0.3.20)
- **Header card overlay** replaces splash screen — animated, responsive, vanishes after load
- **Select/Drag to Copy** — native mouse selection in TUI, auto-copies to clipboard on release
- **O(N) input render** — tall pastes no longer cause quadratic render cost; scroll-to-cursor preserved
- **Emoji cursor rendering** — grapheme cluster extraction for multi-byte emoji highlighting
- **Line navigation in multiline** — Up/Down navigates lines inside recalled multi-line input
- **F12 mouse capture toggle** — toggle native terminal text selection without exiting TUI
- **Async session load** — instant first paint, messages load in background
- **Video attachments in TUI** — pasting a video path emits `<<VID:path>>`, top-right indicator labels each as `Video #N`, chat display rewrites to `[VID: clip.mp4]` (v0.3.17)
- **Thinking content persisted to DB** — captured on both ResponseComplete and IntermediateText events (v0.3.17)
- **Approval policy read at runtime** — loaded from config on every tool request instead of cached at startup (v0.3.17)

### 🔧 Developer Experience
- **Bang operator (`!cmd`)** — run shell commands directly from TUI input, no LLM round-trip (v0.3.1)
- **Full CLI surface**: 20+ subcommands (`/models`, `/approve`, `/compact`, `/rebuild`, `/evolve`, `/new`, `/doctor`, `/btw`, `/mission-control`, `/skills`, `/repo-audit`, etc.)
- **`/btw` parallel agent** — spawn an isolated sub-agent for side tasks while the main conversation continues (v0.3.15)
- **Mission Control** (`/mission-control`) — full-screen dashboard showing RSI inbox, activity log, and cron schedule in one view (v0.3.16)
- **Skills system** (`/skills`) — browse and launch workflow templates with fuzzy-finding, auto-registered as slash commands (v0.3.16)
- **Programmatic `/evolve`** — bypasses LLM, runs update directly (v0.3.1)
- **Auto-update on startup** — `[agent] auto_update = true` silently installs + hot-restarts (v0.3.1)
- **Dynamic tools** — runtime-defined via TOML (HTTP + shell executors)
- **Split panes** — tmux-style parallel sessions with layout persistence
- **Usage Dashboard** — `/usage` command shows daily tokens, cost, active models, session categories, project activity (v0.3.9)
- **Onboarding welcome** — personalized first-time detection with welcome message and guided setup (v0.3.13)
- **Recent file memory** — persists recent file paths across sessions to anchor the agent (v0.3.13)
- **Bash hardening** — rejects interactive commands up-front, short-circuits exact same failing command retries, tilde expansion fixed (v0.3.13)
- **SSH askpass detection** — detects password prompts on remote servers and aborts gracefully instead of hanging (v0.3.16)
- **Async proactive compaction** — at 65% context, compaction runs in background without blocking the chat (v0.3.16)
- **Generic `deliver_api_key` for cron jobs** — HTTP webhook Bearer token auth configurable per-job via `cron_manage` tool (v0.3.18)
- **File paths starting with `/` no longer treated as slash command typos** — `/Users/.../file.pdf yo crabs check this` works correctly (v0.3.18)
- **Truncation continuations no longer trigger provider fallback** — mid-sentence continuations stay on the same provider (v0.3.18)
- **Fallback error reason surfaced in TUI** — when fallback fires, the underlying error shows as a system message (v0.3.18)
- **OpenAI-compatible embedding API** — configure external embedding providers (OpenAI, Ollama, Jina, LM Studio) instead of downloading 300MB GGUF model. Dynamic vector dimensions from API response (v0.3.19)
- **FTS5-only memory mode for VPS** — pure keyword search with zero RAM overhead. Auto-detects VPS environments and configures automatically (v0.3.19)

### 🌐 Browser Automation
- **Full CDP support**: navigate, click, type, screenshot, JS eval, wait for selectors, find elements
- **`browser_find` tool** — enumerate elements by CSS, XPath, text, or aria-label with stable selectors (v0.3.13)
- **`browser_close` tool** — close browser tabs and free CDP sessions, prevents stale page reuse (v0.3.18)
- **Headless or headed** mode, element-specific screenshots
- **Cookie/session persistence** across browser sessions
- **Per-session tab isolation** — no cross-session DOM stomping (v0.3.13)
- **Smart default browser detection** — auto-detects your default Chromium on macOS, Linux, and Windows (v0.3.13)

### 🎥 Video Vision (v0.3.17)
- **`analyze_video` tool** — routes video attachments through Gemini's multimodal API. Inline bytes for files ≤18 MB, resumable Files API upload for larger files
- **Video uploads across all channels** — Slack, Telegram, Discord, WhatsApp, and Trello automatically route video attachments to `analyze_video`
- **`<<VID:path>>` marker** — analogous to `<<IMG:>>` for images. Supports mp4/m4v/mov/webm/mkv/avi/3gp/flv

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
- **2,796+ tests** covering providers, tools, channels, TUI, self-healing, crash recovery, browser automation
- **CI/CD**: GitHub Actions, CodeQL, `cargo audit` security checks, release automation

### 🔧 Built-in Skills (v0.3.17)
- **5 safe built-in skills**: opencli, browser-cdp, a2a-gateway, dynamic-tools, repo-audit
- **SKILLS section** added to help screen and splash integration
- **`/repo-audit` skill** — language-agnostic repository health checks. 5-phase pipeline: language detection → native tool execution → git metrics → AST analysis → scoring + recommendations (v0.3.18)

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
