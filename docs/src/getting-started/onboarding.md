# Onboarding

When you launch OpenCrabs for the first time, the **onboarding wizard** walks you through setup.

## 🎬 Full Onboarding Walkthrough

<video controls width="100%" preload="metadata" playsinline>
  <source src="https://github.com/user-attachments/assets/833dd5e9-3bcc-432a-96ac-3a5bb97b5966" type="video/mp4">
  <track kind="subtitles" src="../assets/advanced-onboard.vtt" srclang="en" label="English" default>
  Your browser does not support the video tag. <a href="https://github.com/user-attachments/assets/833dd5e9-3bcc-432a-96ac-3a5bb97b5966">Watch the video</a>.
</video>

Narrated step-by-step covering both the Quick and Advanced paths below.

## Quick Start (3 minutes)

The fast path gets you chatting with the agent in under 3 minutes.

| Step | Action |
|------|--------|
| 1. Mode | Hit `Enter` on **QuickStart** |
| 2. Workspace | Hit `Enter` to accept the default path |
| 3. Provider | Arrow to your provider (e.g. z.ai), hit `Enter`. Arrow down to select a plan (e.g. Coding), hit `Enter` |
| 4. API Key | Paste your key (`Cmd+V` / `Ctrl+V` / `Cmd+Shift+V` / `Ctrl+Shift+V`), hit `Enter`. Model list loads live |
| 5. Model | Arrow to your model (e.g. gemini-2.5-pro), hit `Enter` |
| 6. Daemon | Arrow to select whether to run as background daemon, hit `Enter` |
| 7. Vibe Check | All checks should show ✅. Hit `Enter` |
| 8. About You | Write something about yourself, the more the agent knows, the better. Hit `Enter` |
| 9. About Agent | Write something about the agent's personality. Hit `Enter` |
| 10. Chat | You're in. Start talking to your agent |

## Advanced Setup (7 minutes)

Full setup with Telegram, local voice, vision, and image generation.

| Step | Action |
|------|--------|
| 1. Mode | Hit `Enter` on **QuickStart** |
| 2. Workspace | Hit `Enter` to accept the default path |
| 3. Provider | Arrow to your provider (e.g. z.ai), hit `Enter`. Arrow down to select a plan, hit `Enter` |
| 4. API Key | Paste your key, hit `Enter`. Model list loads live |
| 5. Model | Arrow to your model, hit `Enter` |
| 6. Channels | Arrow to **Other**, hit `Space` to select Telegram, hit `Enter`. Paste your bot token, follow the instructions to get your chat ID, hit `Enter`. Select mention mode, hit `Enter`. Once it says **Connected**, hit `Enter` again. Arrow down to **Continue**, hit `Enter` |
| 7. STT | Select **Local**, hit `Enter`. Pick model size (e.g. tiny for speed), hit `Enter` |
| 8. TTS | Select **Local** again, hit `Enter`. Pick a voice (e.g. Ryan), hit `Enter`. Wait for the model download, arrow down to **Continue**, hit `Enter` |
| 9. Image | Hit `Space` to select Vision and Image Generation, hit `Enter`. Paste your Gemini API key, hit `Enter` |
| 10. Daemon | Arrow to select whether to run as background daemon, hit `Enter` |
| 11. Vibe Check | All checks should show ✅. Hit `Enter` |
| 12. About You | Write something about yourself, the more the agent knows, the better. Hit `Enter` |
| 13. About Agent | Write something about the agent's personality. Hit `Enter` |
| 14. Chat | You're in. Start talking to your agent |

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
| 8 | **Brain Setup** | Auto-generate SOUL.md from your profile |

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

After onboarding, your agent boots up and introduces itself. It reads its brain files (`SOUL.md`, `AGENTS.md`, `TOOLS.md`) and starts a conversation.

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

The working directory is persisted per-session. Switching sessions restores the directory automatically.
