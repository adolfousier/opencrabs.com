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

The agent can also use CLI tools available on the system through `bash`:

- `gh` — GitHub CLI (PRs, issues, releases)
- `docker` — Container management
- `ssh` — Remote server access
- `node` / `python3` — Script execution
- `curl` — HTTP requests
- `ffmpeg` — Media processing
