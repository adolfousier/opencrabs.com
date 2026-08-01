# Security

## Threat Model

OpenCrabs runs locally on your machine with access to your filesystem and shell. Security focuses on:

1. **API key protection** — Keys never leave your machine except to their respective providers
2. **Network exposure** — Minimal attack surface by default
3. **Tool execution** — Sandboxed with user approval

## API Key Storage

Keys are stored in `~/.opencrabs/keys.toml`:

- File permissions: `600` (owner read/write only)
- Keys are loaded into memory with `zeroize` — zeroed on drop
- Keys are never logged or included in conversation history
- Keys are never sent to any provider other than their own

## Network Security

### A2A Gateway

- Binds to `127.0.0.1` (loopback) by default
- CORS disabled unless explicitly configured
- No authentication built-in — use a reverse proxy for public exposure

### Channel Connections

- All channel APIs use TLS (HTTPS/WSS)
- Telegram: long polling over HTTPS
- Discord: WebSocket with TLS
- Slack: Socket Mode (WebSocket)
- WhatsApp: Noise protocol encryption

## Tool Approval

Tools that modify your system require approval:

- **File writes** — Shows the file path and diff
- **Shell commands** — Shows the exact command before execution
- **Git operations** — Push, commit, branch operations

Auto-approve mode (`--auto-approve`) bypasses this for automation use cases like cron jobs.

## Data Storage

- All data stored locally in `~/.opencrabs/opencrabs.db` (SQLite)
- No telemetry or analytics
- No data sent to OpenCrabs servers (there are none)
- Conversation history stays on your machine

## Reporting Vulnerabilities

If you discover a security vulnerability, please report it responsibly:

- Email: adolfo@meetneura.ai
- Do not open a public issue for security vulnerabilities
- We will acknowledge receipt within 48 hours

## Safety Gates (v0.3.78)

OpenCrabs v0.3.78 introduced a layered safety-gate system:

### TOML Bash Blocklist
Shell commands are checked against a TOML-defined blocklist before execution. Dangerous patterns (recursive deletes on root, raw disk writes, fork bombs) are rejected with a clear error. The blocklist is user-extensible and hot-reloads on file change. See [Configuration](../getting-started/configuration.md#toml-bash-blocklist-v0378) for details.

### Secret Redaction
Quoted secrets in agent output are now **redacted before delivery**. This covers:
- Bearer tokens
- API key patterns
- Colon-token shapes (key: value in YAML-like output)
- Query-param keys
- URL passwords

Redaction fires on all delivery paths (TUI, Telegram, Discord, Slack, WhatsApp), so a secret accidentally echoed by the agent never reaches the chat.

### Brain File Verification
Post-write verification (via brain_verify.toml) ensures brain files maintain their structural integrity after RSI writes. Required anchors (H1 title, Owns header) are checked, and writes that break structure are flagged.
