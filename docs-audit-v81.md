# v0.3.81 Docs Audit

**Release:** v0.3.81 — 2026-08-17 (tag `88798a40`)
**Scope:** 48 commits since v0.3.80, 157 files, +12,219 / -1,906 lines, 6,688 tests
**Docs.opencrabs.com HEAD:** `3bd5b78` — last synced to v0.3.80

---

## 1. v0.3.81 headline features (what shipped)

| # | Feature | Issue | Docs home |
|---|---------|-------|-----------|
| 1 | **keys.toml key-loss fix** — four provider API keys stopped being dropped at runtime; stored-key marker has one owner; poisoned keys self-heal | #1066 | `brain/providers.md` |
| 2 | **In-tree memory store** — qmd dependency dropped, OpenCrabs owns the store (SQLite FTS5 + vector) | #1032 | `brain/memory.md` + `development/architecture.md` |
| 3 | **External index paths** — memory search reads indexes outside the profile directory | #1051, #1055 | `brain/memory.md` |
| 4 | **Mermaid as images in Telegram** — markdown+media route, tables stay native | #1044 | `channels/telegram.md` |
| 5 | **Qwen fixes** — preserve_thinking (reasoning across turns), family-gated thinking knobs, effort tier default, hosted reachability | #1033, #1034, #1040 | `brain/providers.md` |
| 6 | **Early loop detection** — repeating tool rounds caught before the provider rejects | — | `features/self-healing.md` |
| 7 | **Subagent result reporting** — finished agents report back; orphaned status files reconciled | #1036, #1038 | `features/multi-agent.md` |
| 8 | **Telegram hardening** — bot-add detection, owner-only group adds, unauthorised chats not recorded, rich blocks as typed AST | #1041, #1042, #1043, #1058 | `channels/telegram.md` |
| 9 | **Embedding hardening + doctor health** — timeouts, vector gate, non-blocking writes, unembedded sweep | #1062, #1069 | `brain/memory.md` |
| 10 | **RSI engine gate** — autonomous RSI off by default for headless daemons, opt in via `rsi_enabled` | #1063 | `features/self-improvement.md` |
| 11 | **Telegram rate-limit cap** — inline waits capped at 30s; flow footer + section cap | #1064, #1052-1056 | `channels/telegram.md` |
| 12 | **Voice dialog keys** — stops hiding and overwriting stored keys | #1039 | `features/voice.md` |
| 13 | **Self-heal improvements** — fact-based detectors fire the heal, wider command allowlist, dotted commands survive sentence split | #1073, #1074 | `features/self-healing.md` |
| 14 | **Session-routed recovery reports** | #1037 | `features/sessions.md` |
| 15 | **Model picker newest first** | #1057 | (TUI, minor — likely no docs page) |

**No upgrade notes this release** (no template changes, no breaking config).

---

## 2. Carried-over red item from v0.3.80 audit (still open)

| File | Problem | Status in .81 |
|------|---------|---------------|
| `development/architecture.md` ~line 80 | `qmd + llama-cpp-2` in the memory search row — qmd was dropped | **Stale since v0.3.80 audit. MUST fix in this pass** (#1032 made it fully in-tree) |

## 3. Introduction page

- Version highlights block → v0.3.81 items
- Test count bullet: currently "6,300+" (commit 3bd5b78 says 6,383) → **6,688 tests** (gauntlet receipt: 6,659 passed + 29 ignored)

## 4. Landing cards (`src/main.rs`)

v0.3.81 card candidates: in-tree memory store (no qmd), keys.toml key-loss fix, mermaid images in Telegram, Qwen reasoning across turns, external index paths. Style: `(v0.3.81)` suffix per existing convention.

---

## Execution plan (12b-12g)

1. `docs(area)` commits per touched page
2. `landing:` commit for feature cards
3. `docs(intro):` commit for highlights + test count
4. `mdbook build` → `docs: rebuild book for v0.3.81` commit
5. `trunk build --release` compile-verify only (dist/ never committed)
6. `git push origin main` — STOP. No deploy (user deploys).
