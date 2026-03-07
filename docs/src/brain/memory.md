# Memory System

OpenCrabs uses a 3-tier memory system for persistent context across sessions.

## Memory Tiers

### 1. Daily Notes (`memory/YYYY-MM-DD.md`)

Automatic daily files for session-specific observations:

```
~/.opencrabs/memory/2026-03-07.md
```

The agent writes here during conversations — new integrations, bugs fixed, decisions made, server changes.

### 2. Long-term Memory (`MEMORY.md`)

Curated knowledge that persists across all sessions:
- Server details, SSH access, credentials locations
- User preferences and workflows
- Integration configurations
- Lessons learned from debugging

### 3. Semantic Search (`session_search`)

Full-text search across all past sessions stored in SQLite. The agent can query:
- Previous conversations
- Tool execution history
- Past decisions and context

## Memory Search

The agent uses `session_search` for fast memory lookups (~500 tokens) instead of reading full memory files (~15K tokens). This is the primary recall mechanism.

## Context Compaction

When context reaches ~80% capacity, OpenCrabs automatically compacts:

1. Summarizes the conversation so far into a comprehensive continuation document
2. Clears old messages from context
3. Continues with the summary as context

Manual compaction: type `/compact` in chat.

## Auto-Save Triggers

The agent saves to memory when:
- New integrations are connected
- Server/infrastructure changes occur
- Bugs are found and fixed
- New tools are configured
- Credentials are rotated
- Architecture decisions are made
- You say "remember this"
- Errors take >5 minutes to debug

## Brain Files

See [Brain Files](./brain-files.md) for the full list of files the agent reads on startup.
