# Architecture

## High-Level Overview

```
┌─────────────────────────────────────────────────┐
│                    TUI (ratatui)                 │
├────────┬────────┬──────────┬────────────────────┤
│Telegram│Discord │  Slack   │     WhatsApp       │
├────────┴────────┴──────────┴────────────────────┤
│                 Brain (Agent Core)               │
│  ┌──────────┐ ┌──────────┐ ┌──────────────────┐ │
│  │ Providers│ │  Tools   │ │  Memory (3-tier) │ │
│  └──────────┘ └──────────┘ └──────────────────┘ │
├─────────────────────────────────────────────────┤
│              Services / DB (SQLite)              │
├─────────────────────────────────────────────────┤
│         A2A Gateway (axum) │ Cron Scheduler     │
└─────────────────────────────────────────────────┘
```

## Source Layout

```
src/
├── main.rs              # Entry point, CLI parsing
├── lib.rs               # Library root
├── cli/                 # CLI argument parsing (clap)
├── config/              # Configuration types and loading
├── db/                  # SQLite database layer
│   ├── models.rs        # Data models (Session, Message, etc.)
│   └── repository/      # Query functions per entity
├── migrations/          # SQL migration files
├── services/            # Business logic layer
│   └── session.rs       # Session management service
├── brain/               # Agent core
│   ├── agent/           # Agent service, context, tool loop
│   │   └── service/     # Builder, context, helpers, tool_loop
│   ├── provider/        # LLM provider implementations
│   ├── tools/           # 40+ tool implementations
│   └── memory/          # 3-tier memory system
├── tui/                 # Terminal UI (ratatui + crossterm)
│   ├── app/             # App state, input, messaging
│   └── render/          # UI rendering modules
├── channels/            # Messaging platform integrations
│   ├── telegram/        # Teloxide-based bot
│   ├── discord/         # Serenity-based bot
│   ├── slack/           # Slack Socket Mode
│   └── whatsapp/        # WhatsApp Web pairing
├── a2a/                 # Agent-to-Agent gateway (axum)
├── cron/                # Cron job scheduler
├── memory/              # Vector search + FTS5
├── docs/                # Embedded doc templates
├── tests/               # Integration tests
└── benches/             # Criterion benchmarks
```

## Key Crates

| Crate | Purpose |
|-------|---------|
| `ratatui` + `crossterm` | Terminal UI rendering and input |
| `sqlx` (SQLite) | Database with compile-time checked queries |
| `reqwest` | HTTP client for LLM APIs |
| `axum` + `tower-http` | A2A HTTP gateway |
| `crabrace` | Provider registry and routing |
| `teloxide` | Telegram Bot API |
| `serenity` | Discord gateway |
| `slack-morphism` | Slack API |
| `qmd` + `llama-cpp-2` | Memory search (FTS5 + embeddings) |
| `syntect` | Syntax highlighting in TUI |
| `tiktoken-rs` | Token counting |

## Data Flow

1. **Input** arrives from TUI, channel, A2A, or cron trigger
2. **Brain** builds context (system prompt + brain files + memory + conversation)
3. **Provider** streams the LLM response via the selected provider
4. **Tool Loop** executes any tool calls, feeds results back to the LLM
5. **Response** is delivered back to the originating channel
6. **DB** persists messages, token usage, and session state

## Database

SQLite with WAL mode. Tables:

- `sessions` — Session metadata, provider, model, working directory
- `messages` — Conversation history per session
- `usage_ledger` — Permanent token/cost tracking
- `memory_*` — FTS5 and vector tables for semantic memory

Migrations run automatically on startup from `src/migrations/`.

## Concurrency Model

- **Tokio** async runtime with multi-threaded scheduler
- Each channel runs as an independent tokio task
- Sessions are isolated — each has its own conversation state
- Tool execution uses `tokio::task::block_in_place` for sync operations
- A2A gateway runs as a separate axum server task
