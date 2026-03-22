# AI Provider Setup

OpenCrabs supports multiple AI providers. Configure them in `config.toml` and `keys.toml` at `~/.opencrabs/`.

## Anthropic Claude

**Models:** `claude-opus-4-6`, `claude-sonnet-4-5`, `claude-haiku-4-5`, and legacy models — fetched live from the API.

```toml
# keys.toml
[providers.anthropic]
api_key = "sk-ant-..."
```

```toml
# config.toml
[providers.anthropic]
enabled = true
default_model = "claude-sonnet-4-20250514"
```

**Features:** Streaming, tool use, extended thinking, vision, 200K context window.

## OpenAI

**Models:** `GPT-5 Turbo`, `GPT-5`, and others — fetched live.

```toml
# keys.toml
[providers.openai]
api_key = "sk-YOUR_KEY"
```

## OpenRouter — 400+ Models

Access 400+ models from every major provider through a single API key. Includes free models (DeepSeek-R1, Llama 3.3, Gemma 2, Mistral 7B).

```toml
# keys.toml
[providers.openrouter]
api_key = "sk-or-YOUR_KEY"
```

Get a key at [openrouter.ai/keys](https://openrouter.ai/keys). Model list is fetched live — no binary update needed when new models are added.

## Google Gemini

**Models:** `gemini-2.5-flash`, `gemini-2.0-flash`, `gemini-1.5-pro` — fetched live.

```toml
# keys.toml
[providers.gemini]
api_key = "AIza..."
```

```toml
# config.toml
[providers.gemini]
enabled = true
default_model = "gemini-2.5-flash"
```

**Features:** Streaming, tool use, vision, 1M+ token context window.

> Gemini also powers the separate image generation and vision tools. See [Image Generation & Vision](./images.md).

## GitHub Copilot

Use your existing GitHub Copilot subscription — no separate API charges. Authenticates via OAuth device flow.

```toml
# config.toml
[providers.github_copilot]
enabled = true
```

**Setup:** Run `/onboard:providers` → select GitHub Copilot → follow the device code flow at `github.com/login/device`. Models are fetched live from the Copilot API.

**Requirements:** An active [GitHub Copilot](https://github.com/features/copilot) subscription (Individual, Business, or Enterprise).

## z.ai (Zhipu AI)

**Models:** GLM-4-Plus, GLM-4-Flash, GLM-4-0520, CodeGeeX — fetched live. Two endpoint types: General API and Coding API.

```toml
# keys.toml
[providers.zai]
api_key = "your-api-key"
```

```toml
# config.toml
[providers.zai]
enabled = true
default_model = "glm-4-plus"
```

Get your API key at [open.bigmodel.cn](https://open.bigmodel.cn).

## Claude CLI

Use your existing Claude Code subscription through the local `claude` binary — no separate API key needed. Supports streaming and extended thinking.

```toml
# config.toml
[providers.claude_cli]
enabled = true
```

**Requirements:** The `claude` CLI must be installed and authenticated. Models are detected automatically.

## OpenCode CLI

Use the local `opencode` binary for free LLM completions — no API key or subscription needed. Supports NDJSON streaming and extended thinking.

```toml
# config.toml
[providers.opencode_cli]
enabled = true
```

**Requirements:** The `opencode` binary must be installed and available in your PATH. Models are fetched live via `opencode models`.

## MiniMax

**Models:** `MiniMax-M2.7`, `MiniMax-M2.5`, `MiniMax-M2.1`, `MiniMax-Text-01`

```toml
# keys.toml
[providers.minimax]
api_key = "your-api-key"
```

Get your API key from [platform.minimax.io](https://platform.minimax.io). Model list comes from `config.toml` (no `/models` endpoint).

## Custom (OpenAI-Compatible)

Use for Ollama, LM Studio, LocalAI, Groq, or any OpenAI-compatible API.

```toml
# config.toml
[providers.custom.lm_studio]
enabled = true
base_url = "http://localhost:1234/v1"
default_model = "qwen2.5-coder-7b-instruct"
models = ["qwen2.5-coder-7b-instruct", "llama-3-8B"]
```

> **Local LLMs:** No API key needed — just set `base_url` and `default_model`.
>
> **Remote APIs (Groq, etc.):** Add the key in `keys.toml`:
> ```toml
> [providers.custom.groq]
> api_key = "your-api-key"
> ```

### Multiple Custom Providers

Define as many as you need with different names:

```toml
[providers.custom.lm_studio]
enabled = true
base_url = "http://localhost:1234/v1"
default_model = "qwen2.5-coder-7b-instruct"

[providers.custom.ollama]
enabled = false
base_url = "http://localhost:11434/v1"
default_model = "mistral"
```

### Free Prototyping with NVIDIA API

[Kimi K2.5](https://build.nvidia.com/moonshotai/kimi-k2.5) is available for free on the NVIDIA API Catalog — no billing required.

```toml
# config.toml
[providers.custom.nvidia]
enabled = true
base_url = "https://integrate.api.nvidia.com/v1"
default_model = "moonshotai/kimi-k2.5"
```

```toml
# keys.toml
[providers.custom.nvidia]
api_key = "nvapi-..."
```

## Fallback Provider Chain

Configure automatic failover when the primary provider fails (rate limits, outages, errors). Fallbacks are tried in order until one succeeds.

```toml
# config.toml
[providers.fallback]
enabled = true
providers = ["openrouter", "anthropic"]  # Tried in order on failure
```

Each fallback provider must have its API key configured in `keys.toml`. Both `complete()` and `stream()` calls are retried transparently — no changes needed downstream.

Single fallback shorthand:

```toml
[providers.fallback]
enabled = true
provider = "openrouter"
```

Or just ask your Crab: *"Set up fallback providers using openrouter and anthropic"* — it will configure `config.toml` for you at runtime.

## Vision Model

When your default chat model doesn't support vision, set `vision_model` to a vision-capable model on the same provider. This registers a vision tool that the agent can call — it sends the image to the vision model, gets a description back, and the chat model uses that context to reply.

```toml
# config.toml
[providers.minimax]
enabled = true
default_model = "MiniMax-M2.5"
vision_model = "MiniMax-Text-01"  # Agent calls vision tool → this model describes image → M2.5 replies
```

```toml
[providers.openai]
enabled = true
default_model = "gpt-5-nano"
vision_model = "gpt-5-nano"
```

MiniMax auto-configures `vision_model = "MiniMax-Text-01"` on first run. You can also ask your Crab to set it up: *"Configure vision model for MiniMax"* — it will update `config.toml` at runtime.

This is separate from the [Gemini image tools](./images.md) which provide dedicated `generate_image` and `analyze_image` tools.

## Per-Session Providers

Each session remembers its provider and model. Switch to Claude in one session, Gemini in another — switching sessions restores the provider automatically.
