# First Session

When you launch OpenCrabs for the first time, the **onboarding wizard** walks you through setup.

## Onboarding Flow

1. **Provider selection** — Choose your AI provider (Anthropic, OpenAI, Gemini, OpenRouter, or custom)
2. **API key** — Enter your API key
3. **Model selection** — Pick a default model (fetched live from the provider)
4. **Channel setup** (optional) — Connect Telegram, Discord, Slack, or WhatsApp
5. **Image tools** (optional) — Configure Gemini for image generation and vision

After onboarding, your agent boots up and introduces itself. It reads its brain files (`SOUL.md`, `IDENTITY.md`, `AGENTS.md`, `TOOLS.md`) and starts a conversation.

## Bootstrap

On the very first run, the agent goes through a **bootstrap** phase:

- Gets to know you (name, preferences, work style)
- Establishes its identity (name, personality, emoji)
- Opens `SOUL.md` together to discuss values
- Sets up `USER.md` with your profile

The bootstrap file (`BOOTSTRAP.md`) deletes itself when complete.

## Key Commands

| Command | Description |
|---------|-------------|
| `/help` | Show all available commands |
| `/models` | Switch provider or model |
| `/new` | Create a new session |
| `/sessions` | Switch between sessions |
| `/cd` | Change working directory |
| `/compact` | Manually compact context |
| `/evolve` | Download latest version |
| `/rebuild` | Build from source |
| `/approve` | Set approval policy |

## Approval Modes

Control how much autonomy the agent has:

| Mode | Behavior |
|------|----------|
| `/approve` | Ask before every tool use (default) |
| `/approve auto` | Auto-approve for this session |
| `/approve yolo` | Auto-approve always (persists) |

## Working Directory

The agent operates within a working directory for file operations. Change it with:
- `/cd` command in chat
- Directory picker in the TUI (Tab to select)
- `config_manager set_working_directory` tool

The working directory is persisted per-session — switching sessions restores the directory automatically.
