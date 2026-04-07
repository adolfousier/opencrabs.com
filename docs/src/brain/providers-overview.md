# Supported AI Providers

OpenCrabs supports 11+ AI providers out of the box. Switch between them at any time via `/models` in the TUI or any channel.

| Provider | Auth | Models | Streaming | Tools | Notes |
|----------|------|--------|-----------|-------|-------|
| [Anthropic Claude](./providers.md#anthropic-claude) | API key | Claude Opus 4.6, Sonnet 4.5, Haiku 4.5 | Yes | Yes | Extended thinking, 200K context |
| [OpenAI](./providers.md#openai) | API key | GPT-5 Turbo, GPT-5, o3/o4-mini | Yes | Yes | Models fetched live |
| [GitHub Copilot](./providers.md#github-copilot) | OAuth | GPT-4o, Claude Sonnet 4+ | Yes | Yes | Uses your Copilot subscription — no API charges |
| [OpenRouter](./providers.md#openrouter--400-models) | API key | 400+ models | Yes | Yes | Free models available. Reasoning output support (Qwen 3.6 Plus, etc.) |
| [Google Gemini](./providers.md#google-gemini) | API key | Gemini 2.5 Flash, 2.0, 1.5 Pro | Yes | Yes | 1M+ context, vision, image generation |
| [MiniMax](./providers.md#minimax) | API key | M2.7, M2.5, M2.1, Text-01 | Yes | Yes | Competitive pricing, auto-configured vision |
| [z.ai GLM](./providers.md#zai-zhipu-ai) | API key | GLM-4.5 through GLM-5 Turbo | Yes | Yes | General API + Coding API endpoints |
| [Claude CLI](./providers.md#claude-cli) | CLI auth | Via `claude` binary | Yes | Yes | Uses your Claude Code subscription |
| [Qwen Code CLI](./providers.md#qwen-code-cli) | CLI auth (OAuth) | qwen3-coder-plus, qwen3.5-plus, qwen3.6-plus | Yes | Yes | 1,000 free requests/day via Qwen OAuth |
| [OpenCode CLI](./providers.md#opencode-cli) | None | Free models (Mimo, etc.) | Yes | Yes | Free — no API key or subscription needed |
| [Custom](./providers.md#custom-openai-compatible) | Optional | Any | Yes | Yes | Ollama, LM Studio, Groq, NVIDIA, any OpenAI-compatible API |

## How It Works

- **One provider active at a time** per session — switch with `/models`
- **Per-session isolation** — each session remembers its own provider and model. Changing provider in the TUI does not affect other active sessions (Telegram, Discord, Slack)
- **Fallback chain** — configure automatic failover when the primary provider goes down
- **Models fetched live** — no binary update needed when providers add new models
- **Function calling detection** — OpenCrabs detects when a model doesn't support tool use and warns you with a model switch suggestion, rather than silently failing
- **`tool_choice: "auto"`** — sent automatically for OpenAI-compatible providers when tools are active, enabling function calling on models that require explicit opt-in

## OpenRouter Reasoning

For models that support extended reasoning (e.g. Qwen 3.6 Plus), OpenCrabs sends `include_reasoning: true` automatically when using OpenRouter. Thinking/reasoning output is displayed in collapsible sections:

```
▶ Thinking... (click to expand)
  The user wants to refactor...
```

Reasoning text wraps to screen width instead of truncating.

See [Provider Setup](./providers.md) for configuration details and API key setup.
