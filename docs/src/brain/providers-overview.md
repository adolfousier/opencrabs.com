# Supported AI Providers

OpenCrabs supports 10+ AI providers out of the box. Switch between them at any time via `/models` in the TUI or any channel.

| Provider | Auth | Models | Streaming | Tools | Notes |
|----------|------|--------|-----------|-------|-------|
| [Anthropic Claude](./providers.md#anthropic-claude) | API key | Claude Opus 4.6, Sonnet 4.5, Haiku 4.5 | Yes | Yes | Extended thinking, 200K context |
| [OpenAI](./providers.md#openai) | API key | GPT-5 Turbo, GPT-5, o3/o4-mini | Yes | Yes | Models fetched live |
| [GitHub Copilot](./providers.md#github-copilot) | OAuth | GPT-4o, Claude Sonnet 4+ | Yes | Yes | Uses your Copilot subscription — no API charges |
| [OpenRouter](./providers.md#openrouter--400-models) | API key | 400+ models | Yes | Yes | Free models available (DeepSeek-R1, Llama 3.3, etc.) |
| [Google Gemini](./providers.md#google-gemini) | API key | Gemini 2.5 Flash, 2.0, 1.5 Pro | Yes | Yes | 1M+ context, vision, image generation |
| [MiniMax](./providers.md#minimax) | API key | M2.7, M2.5, M2.1, Text-01 | Yes | Yes | Competitive pricing, auto-configured vision |
| [z.ai GLM](./providers.md#zai-zhipu-ai) | API key | GLM-4.5 through GLM-5 Turbo | Yes | Yes | General API + Coding API endpoints |
| [Claude CLI](./providers.md#claude-cli) | CLI auth | Via `claude` binary | Yes | Yes | Uses your Claude Code subscription |
| [OpenCode CLI](./providers.md#opencode-cli) | None | Free models (Mimo, etc.) | Yes | Yes | Free — no API key or subscription needed |
| [Custom](./providers.md#custom-openai-compatible) | Optional | Any | Yes | Yes | Ollama, LM Studio, Groq, NVIDIA, any OpenAI-compatible API |

## How It Works

- **One provider active at a time** per session — switch with `/models`
- **Per-session memory** — each session remembers its provider and model
- **Fallback chain** — configure automatic failover when the primary provider goes down
- **Models fetched live** — no binary update needed when providers add new models

See [Provider Setup](./providers.md) for configuration details and API key setup.
