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

### Scoped Search (v0.3.80)

`memory_search` takes a `scope` that picks which corpus to search (#1020):

| Scope | Corpus | Use it for |
|-------|--------|------------|
| `memory` (default) | Daily logs | History: what happened, when, what was decided |
| `brain` | Brain files (SOUL, USER, AGENTS, TOOLS, CODE, SECURITY, MEMORY, BOOT, HEARTBEAT) | Rules and policy: does a rule about this ALREADY exist, and which file owns it |
| `all` | Both | "Have I ever written about this anywhere" |

Picking the wrong scope is the usual reason a search comes back with nothing useful. Daily logs outnumber brain files and reuse the same words for unrelated things, so searching `memory` for a rule usually fails: history outranks policy and you get confident irrelevant hits. Search `brain` before appending any rule: a hit tells you WHICH file owns it, and then `load_brain_file` with a `query` reads the whole section.

### Chunked Retrieval and Index-on-Write (v0.3.80)

Memory retrieval was rebuilt around chunking (#998-1002, #1018):

- **Documents are chunked before embedding**, so chunks past the first are searchable. Previously a long document effectively hid everything after its opening.
- **Lexical hits are narrowed to the matching chunk**, so keyword results point at the relevant slice instead of the whole document.
- **The index updates on write**: memory is searchable immediately, not from a boot-time snapshot. Brain files are indexed into the brain collection on write too.
- **The store resolves per profile**, so profiles never share or cache each other's memory index.
- **The chunker is multi-byte safe**: no more panics on UTF-8 boundaries.
- **MEMORY.md recall is ranked with BM25** instead of shared-word counts (#996), and recall folds Latin diacritics so accented queries match.

### Embedding Modes

OpenCrabs supports three embedding configurations:

1. **Local GGUF** (default) — downloads a 300MB embedding model and runs it locally via llama.cpp
2. **OpenAI-compatible API** — configure external embedding providers (OpenAI `text-embedding-3-small`, Ollama `nomic-embed-text`, Jina, LM Studio, or any `/v1/embeddings` endpoint) via `[memory.embedding]` config with `url`, `model`, `api_key`, `dimensions`
3. **FTS5-only** — pure keyword search with zero RAM overhead. Set `[memory] vector_enabled = false`. Auto-detects VPS environments and configures automatically

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

## Proactive Memory Surfacing (v0.3.76)

The agent now **surfaces relevant memory without being asked**. When a conversation topic matches something stored in MEMORY.md or daily notes, the relevant context is loaded and injected automatically. Previously, memory was only recalled when the agent explicitly called `memory_search` or `load_brain_file`. Now the system proactively checks for relevant context on each turn, so the agent brings up past decisions, server details, or preferences that apply to the current conversation without you having to say "check your memory."

## Brain Files

See [Brain Files](./brain-files.md) for the full list of files the agent reads on startup.
