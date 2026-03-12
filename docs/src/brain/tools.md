# Built-in Tools

OpenCrabs ships with 40+ tools available to the agent out of the box.

## File Operations

| Tool | Parameters | Description |
|------|-----------|-------------|
| `ls` | `path` | List directory contents |
| `glob` | `pattern`, `path` | Find files by glob pattern |
| `grep` | `pattern`, `path`, `include` | Search file contents with regex |
| `read_file` | `path`, `line_start`, `line_end` | Read file contents |
| `edit_file` | `path`, `old_string`, `new_string` | Edit files with search/replace |
| `write_file` | `path`, `content` | Write new files |

## Code Execution

| Tool | Parameters | Description |
|------|-----------|-------------|
| `bash` | `command`, `timeout` | Execute shell commands |
| `execute_code` | `language`, `code` | Run code in sandboxed environment |

## Web & Network

| Tool | Parameters | Description |
|------|-----------|-------------|
| `web_search` | `query` | Search the web (Brave Search) |
| `http_request` | `method`, `url`, `headers`, `body` | Make HTTP requests |

## Session & Memory

| Tool | Parameters | Description |
|------|-----------|-------------|
| `session_search` | `query`, `limit` | Semantic search across sessions |
| `session_context` | `action` | Read/write session context |
| `task_manager` | `action`, various | Manage plans and tasks |

## Image

| Tool | Parameters | Description |
|------|-----------|-------------|
| `generate_image` | `prompt`, `filename` | Generate images via Gemini |
| `analyze_image` | `image`, `question` | Analyze images via Gemini vision |

## Channel Integrations

| Tool | Parameters | Description |
|------|-----------|-------------|
| `telegram_send` | `action`, various | Telegram operations (19 actions) |
| `discord_connect` | `action`, various | Discord operations (17 actions) |
| `slack_send` | `action`, various | Slack operations (17 actions) |
| `trello_connect` | `action`, various | Trello operations (22 actions) |

## System

| Tool | Parameters | Description |
|------|-----------|-------------|
| `slash_command` | `command`, `args` | Execute slash commands (/cd, /compact, etc.) |
| `config_manager` | `action`, various | Read/write config, manage commands |
| `evolve` | `check_only` | Download latest release |
| `rebuild` | — | Build from source and restart |
| `plan` | `action`, various | Create and manage execution plans |

## System CLI Tools

OpenCrabs runs in a TUI with full terminal access. The agent can execute **any CLI tool** installed on the host via the `bash` tool — no plugins, no wrappers. If it's on your system, the agent can use it. Common ones:

| Tool | Purpose | Check |
|------|---------|-------|
| `gh` | GitHub CLI — issues, PRs, repos, releases, actions | `gh --version` |
| `gog` | Google CLI — Gmail, Calendar (OAuth) | `gog --version` |
| `docker` | Container management | `docker --version` |
| `ssh` | Remote server access | `ssh -V` |
| `node` | Run JavaScript/TypeScript tools | `node --version` |
| `python3` | Run Python scripts and tools | `python3 --version` |
| `ffmpeg` | Audio/video processing | `ffmpeg -version` |
| `curl` | HTTP requests (fallback when `http_request` insufficient) | `curl --version` |

### GitHub CLI (gh)

Authenticated GitHub CLI for full repo management:

```bash
gh issue list / view / create / close / comment
gh pr list / view / create / merge / checks
gh release list / create
gh run list / view / watch
```

### Google CLI (gog)

OAuth-authenticated Google Workspace CLI. Supports Gmail and Calendar:

```bash
gog calendar events --max 10
gog gmail search "is:unread" --max 20
gog gmail send --to user@email.com --subject "Subject" --body "Body"
```

Requires `GOG_KEYRING_PASSWORD` and `GOG_ACCOUNT` env vars.

## Companion Tools

### SocialCrabs — Social Media Automation

[SocialCrabs](https://github.com/adolfousier/socialcrabs) is a social media automation tool with human-like behavior simulation (Playwright). Supports **Twitter/X**, **Instagram**, and **LinkedIn**.

The agent calls SocialCrabs CLI commands via `bash`:

```bash
node dist/cli.js x tweet "Hello world"
node dist/cli.js x mentions -n 5
node dist/cli.js ig like <post-url>
node dist/cli.js linkedin connect <profile-url>
```

Read operations are safe. Write operations (tweet, like, follow, comment) require explicit user approval.

### WhisperCrabs — Floating Voice-to-Text

[WhisperCrabs](https://github.com/adolfousier/whispercrabs) is a floating voice-to-text widget controllable via D-Bus. Click to record, click to stop, text goes to clipboard. The agent can start/stop recording, switch providers, and view transcription history via D-Bus commands.
