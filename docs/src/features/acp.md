# ACP — Editor Integration

**ACP server mode** lets agent-aware editors (Zed, MonoCode, any ACP client) drive OpenCrabs as a backend agent over the [Agent Client Protocol](https://agentclientprotocol.com). The editor spawns `opencrabs acp` as a child process and speaks JSON-RPC 2.0 over stdio, so your editor session and your channels share the same brain, tools, memory, and provider configuration (#1540).

## Running

```bash
opencrabs acp            # default model from config
opencrabs acp --model anthropic/claude-sonnet-4-20250514
```

Point your editor's ACP/agent client at the `opencrabs acp` command and it appears as an available agent. One child process serves one conversation thread.

## Protocol

- **Framing**: JSON-RPC 2.0, newline-delimited (NDJSON) over stdio
- **`session/prompt`**: runs the full tool loop for a turn; the reply arrives when the turn settles
- **`session/update`**: progress notifications stream back while the turn works (tool calls, output chunks)
- **`session/request_permission`**: tool approvals round-trip to the editor, so the client's permission UI gates dangerous calls the same way channel approvals do

stdout is the protocol channel: nothing but JSON-RPC frames is ever written to it. Logs go to the daily log file (or stderr in debug builds), never stdout.

## Why it matters

Before ACP mode, driving OpenCrabs from an editor meant copy-pasting between the editor and a channel. With ACP mode the editor is a first-class surface: same providers, same skill gating, same plan approvals, same memory. Configuration is shared with the rest of OpenCrabs; there is no separate ACP config section.
