# OpenCrabs

**The autonomous AI agent. Single Rust binary. Every channel.**

OpenCrabs is an open-source AI agent that runs in your terminal. It connects to any LLM provider, orchestrates 40+ built-in tools, and works from Telegram, Discord, Slack, WhatsApp, or the built-in TUI.

## Why OpenCrabs?

| | OpenCrabs | Node.js Frameworks |
|---|---|---|
| **Binary size** | 17-22 MB | 1 GB+ (node_modules) |
| **Runtime deps** | Zero | Node.js + npm + native modules |
| **API keys** | `keys.toml` (local file) | `.env` (process env pollution) |
| **Data residency** | Local SQLite | Varies |
| **Memory safety** | Rust (compile-time) | JavaScript (runtime) |

## Core Capabilities

- **Multi-provider AI** — Anthropic Claude, OpenAI, Google Gemini, OpenRouter (400+ models), MiniMax, Ollama, LM Studio, or any OpenAI-compatible API
- **Every channel** — Telegram, Discord, Slack, WhatsApp, or the built-in TUI
- **40+ tools** — File ops, bash, web search, code execution, image gen, document parsing, and more
- **Persistent memory** — 3-tier memory system: daily notes, long-term memory, semantic search
- **Self-evolving** — Type `/evolve` to download the latest version, or `/rebuild` to build from source
- **Agent-to-Agent** — Built-in A2A gateway for peer-to-peer agent communication
- **Cron jobs** — Schedule isolated or main-session tasks with cron expressions
- **Voice** — Speech-to-text (WhisperCrabs) and text-to-speech (ElevenLabs, OpenAI, Coqui)
- **Plans** — Structured multi-step task planning with approval workflow

## Quick Links

- [GitHub Repository](https://github.com/adolfousier/opencrabs)
- [Releases](https://github.com/adolfousier/opencrabs/releases)
- [Issues](https://github.com/adolfousier/opencrabs/issues)
- [Changelog](./changelog.md)
