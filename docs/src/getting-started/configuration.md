# Configuration

OpenCrabs uses two config files stored at `~/.opencrabs/`:

| File | Purpose |
|------|---------|
| `config.toml` | Provider settings, features, channel connections |
| `keys.toml` | API keys and secrets (never committed to git) |

## Workspace Layout

```
~/.opencrabs/
├── config.toml          # Main configuration
├── keys.toml            # API keys (gitignored)
├── commands.toml        # Custom slash commands
├── opencrabs.db         # SQLite database
├── SOUL.md              # Agent personality
├── IDENTITY.md          # Agent identity
├── USER.md              # Your profile
├── MEMORY.md            # Long-term memory
├── AGENTS.md            # Agent behavior docs
├── TOOLS.md             # Tool reference
├── SECURITY.md          # Security policies
├── HEARTBEAT.md         # Periodic check tasks
├── memory/              # Daily memory notes
│   └── YYYY-MM-DD.md
├── images/              # Generated images
├── logs/                # Application logs
└── skills/              # Custom skills/plugins
```

## Provider Configuration

See [Provider Setup](../brain/providers.md) for detailed provider configuration.

**Quick example** — add Anthropic:

```toml
# config.toml
[providers.anthropic]
enabled = true
default_model = "claude-sonnet-4-20250514"
```

```toml
# keys.toml
[providers.anthropic]
api_key = "sk-ant-..."
```

## Provider Priority

When multiple providers are enabled, the first one found in this order is used for new sessions:

MiniMax > OpenRouter > Anthropic > OpenAI > Gemini > Custom

Each session remembers which provider and model it was using. Switch providers per-session via `/models`.

## Feature Flags

```toml
# config.toml
[agent]
working_directory = "/path/to/default/dir"
thinking = "on"                # "on", "off", or "budget_XXk"

[a2a]
enabled = false
bind = "127.0.0.1"
port = 18790

[image.generation]
enabled = true
model = "gemini-3.1-flash-image-preview"

[image.vision]
enabled = true
model = "gemini-3.1-flash-image-preview"
```
