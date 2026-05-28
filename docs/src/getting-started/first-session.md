# First Session

When you launch OpenCrabs for the first time, the **onboarding wizard** walks you through setup.

## Onboarding Flow

The wizard is a keyboard-driven TUI with 8 steps. Navigate with arrow keys, `Tab` to advance, `Esc` to go back.

| Step | Screen | What you do |
|------|--------|-------------|
| 1 | **Mode Select** | Choose QuickStart (skip channels) or Advanced |
| 2 | **Workspace** | Pick a working directory for file operations |
| 3 | **Provider & Auth** | Select provider → paste API key → pick model (fetched live) |
| 4 | **Channels** | Space to toggle channels on/off → Enter on each to configure |
| 5 | **Voice** | STT provider (Groq, local Whisper, or off) + TTS voice |
| 6 | **Image** | Vision toggle + generation model + API key |
| 7 | **Daemon** | Install background daemon (optional) |
| 8 | **Brain Setup** | Auto-generate SOUL.md, IDENTITY.md from your profile |

### Channel Setup (Step 4)

The channels screen lists 5 integrations: Telegram, Discord, WhatsApp, Slack, Trello.

- `Space` toggles a channel on/off
- `Enter` on an enabled channel opens its setup screen (token, IDs, allowlists)
- `Enter` on **Continue** or `Tab` skips to the next step
- Each channel setup screen has a **Test Connection** button

See [Channels Overview](../channels/overview.md) for the full navigation guide.

### Re-running Setup

You can jump to any step without re-running the full wizard:

| Command | Step |
|---------|------|
| `/onboard` | Full wizard |
| `/onboard:provider` | Provider & model selection |
| `/onboard:channels` | Channel picker |
| `/onboard:voice` | Voice setup |
| `/onboard:image` | Image setup |
| `/onboard:brain` | Brain file generation |

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
